//! Type categorization for organizing generated Salesforce types into logical modules.
//!
//! This module defines how types are grouped and which feature flags enable them.

use std::collections::HashMap;

// NOTE: We intentionally avoid generating `settings/mod.rs` that contains `pub mod settings;`
// because Clippy treats that as `module_inception` and it will fail CI under `-D warnings`.
// To keep the crate warning-free, the settings category module file is named `org_settings.rs`
// (and `settings/mod.rs` will re-export it).

/// A category of Salesforce types
#[derive(Debug, Clone)]
pub struct TypeCategory {
    /// Category name (used for module naming)
    pub name: &'static str,
    /// Feature flag name in Cargo.toml
    pub feature: &'static str,
    /// Module path relative to src/ (e.g., "settings/security.rs")
    pub module_path: &'static str,
    /// Description for documentation
    pub description: &'static str,
    /// Patterns to match type names (supports * as wildcard)
    pub patterns: &'static [&'static str],
    /// Explicit type names to include (exact match)
    pub explicit_types: &'static [&'static str],
}

impl TypeCategory {
    /// Check if a type name matches this category
    pub fn matches(&self, type_name: &str) -> bool {
        // Check explicit types first
        if self.explicit_types.contains(&type_name) {
            return true;
        }

        // Check patterns
        for pattern in self.patterns {
            if pattern_matches(pattern, type_name) {
                return true;
            }
        }

        false
    }
}

/// Simple glob-like pattern matching (supports * as wildcard)
fn pattern_matches(pattern: &str, name: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    if let Some(suffix) = pattern.strip_prefix('*') {
        // *Settings matches anything ending in Settings
        return name.ends_with(suffix);
    }

    if let Some(prefix) = pattern.strip_suffix('*') {
        // Permission* matches anything starting with Permission
        return name.starts_with(prefix);
    }

    if pattern.contains('*') {
        // Handle *middle* patterns
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            return name.starts_with(parts[0]) && name.ends_with(parts[1]);
        }
    }

    // Exact match
    pattern == name
}

/// All defined type categories
pub const CATEGORIES: &[TypeCategory] = &[
    // ==================== SETTINGS ====================
    TypeCategory {
        name: "settings",
        feature: "settings",
        module_path: "settings/org_settings.rs",
        description: "Org settings types (~227 types) for org context and scratch org definitions",
        patterns: &["*Settings"],
        explicit_types: &[],
    },
    // ==================== CORE METADATA ====================
    TypeCategory {
        name: "objects",
        feature: "objects",
        module_path: "metadata/objects.rs",
        description: "Custom objects, fields, and related schema types",
        patterns: &["CustomObject*", "CustomField*", "Index*", "ActionOverride*"],
        explicit_types: &[
            "CustomObject",
            "CustomField",
            "ValidationRule",
            "BusinessProcess",
            "CompactLayout",
            "FieldSet",
            "HistoryRetentionPolicy",
            "ListView",
            "RecordType",
            "SearchLayouts",
            "SharingReason",
            "WebLink",
            "CustomValue",
            "StandardValue",
            "ValueSet",
            "ValueSetValuesDefinition",
            "GlobalValueSet",
            "GlobalValueSetTranslation",
            "StandardValueSet",
            "StandardValueSetTranslation",
        ],
    },
    TypeCategory {
        name: "layouts",
        feature: "layouts",
        module_path: "metadata/layouts.rs",
        description: "Page layouts and UI configuration",
        patterns: &["Layout*", "MiniLayout*", "RelatedList*"],
        explicit_types: &[
            "Layout",
            "LayoutSection",
            "LayoutColumn",
            "LayoutItem",
            "MiniLayout",
            "RelatedListItem",
            "SummaryLayout",
            "SummaryLayoutItem",
            "PlatformActionList",
            "PlatformActionListItem",
            "QuickActionList",
            "QuickActionListItem",
            "FeedLayout",
            "FeedLayoutFilter",
            "FeedLayoutComponent",
        ],
    },
    TypeCategory {
        name: "permissions",
        feature: "permissions",
        module_path: "metadata/permissions.rs",
        description: "Permission sets, profiles, and sharing rules",
        patterns: &[
            "PermissionSet*",
            "Profile*",
            "Sharing*",
            "MutingPermissionSet*",
        ],
        explicit_types: &[
            "PermissionSet",
            "PermissionSetGroup",
            "PermissionSetLicense",
            "Profile",
            "ProfileActionOverride",
            "ProfileApplicationVisibility",
            "ProfileApexClassAccess",
            "ProfileApexPageAccess",
            "ProfileCustomMetadataTypeAccess",
            "ProfileCustomPermissions",
            "ProfileCustomSettingAccess",
            "ProfileExternalDataSourceAccess",
            "ProfileFieldLevelSecurity",
            "ProfileFlowAccess",
            "ProfileLayoutAssignment",
            "ProfileLoginHours",
            "ProfileLoginIpRange",
            "ProfileObjectPermissions",
            "ProfileRecordTypeVisibility",
            "ProfileTabVisibility",
            "ProfileUserPermission",
            "SharingRules",
            "SharingCriteriaRule",
            "SharingOwnerRule",
            "SharingTerritoryRule",
            "SharingGuestRule",
            "SharingSet",
        ],
    },
    TypeCategory {
        name: "flows",
        feature: "flows",
        module_path: "metadata/flows.rs",
        description: "Flow definitions and process automation",
        patterns: &["Flow*"],
        explicit_types: &[
            "Flow",
            "FlowActionCall",
            "FlowApexPluginCall",
            "FlowAssignment",
            "FlowChoice",
            "FlowCollectionProcessor",
            "FlowCondition",
            "FlowConnector",
            "FlowConstant",
            "FlowDecision",
            "FlowDynamicChoiceSet",
            "FlowElement",
            "FlowFormula",
            "FlowInputFieldAssignment",
            "FlowInputValidationRule",
            "FlowLoop",
            "FlowMetadataValue",
            "FlowNode",
            "FlowOutputFieldAssignment",
            "FlowRecordCreate",
            "FlowRecordDelete",
            "FlowRecordFilter",
            "FlowRecordLookup",
            "FlowRecordUpdate",
            "FlowRule",
            "FlowSchedule",
            "FlowScheduledPath",
            "FlowScreen",
            "FlowScreenField",
            "FlowScreenFieldInputParameter",
            "FlowScreenFieldOutputParameter",
            "FlowScreenRule",
            "FlowScreenRuleAction",
            "FlowStage",
            "FlowStart",
            "FlowStep",
            "FlowSubflow",
            "FlowSubflowInputAssignment",
            "FlowSubflowOutputAssignment",
            "FlowTextTemplate",
            "FlowTransform",
            "FlowVariable",
            "FlowWait",
            "FlowWaitEvent",
            "ProcessFlowMigration",
        ],
    },
    TypeCategory {
        name: "apex",
        feature: "apex",
        module_path: "metadata/apex.rs",
        description: "Apex classes, triggers, and components",
        patterns: &["Apex*"],
        explicit_types: &[
            "ApexClass",
            "ApexComponent",
            "ApexPage",
            "ApexTrigger",
            "ApexTestSuite",
            "ApexEmailNotifications",
            "StaticResource",
        ],
    },
    TypeCategory {
        name: "lwc",
        feature: "lwc",
        module_path: "metadata/lwc.rs",
        description: "Lightning Web Components and Aura bundles",
        patterns: &["LightningComponent*", "Aura*"],
        explicit_types: &[
            "LightningComponentBundle",
            "AuraDefinitionBundle",
            "LightningExperienceTheme",
            "LightningMessageChannel",
            "LightningOnboardingConfig",
        ],
    },
    TypeCategory {
        name: "automation",
        feature: "automation",
        module_path: "metadata/automation.rs",
        description: "Workflow rules, approval processes, and automation",
        patterns: &["Workflow*", "Approval*"],
        explicit_types: &[
            "WorkflowRule",
            "WorkflowAction",
            "WorkflowAlert",
            "WorkflowFieldUpdate",
            "WorkflowFlowAction",
            "WorkflowKnowledgePublish",
            "WorkflowOutboundMessage",
            "WorkflowSend",
            "WorkflowTask",
            "WorkflowTimeTrigger",
            "ApprovalProcess",
            "ApprovalAction",
            "ApprovalEntryCriteria",
            "ApprovalPageField",
            "ApprovalStep",
            "ApprovalStepApprover",
            "ApprovalStepRejectBehavior",
            "ApprovalSubmitter",
            "AssignmentRule",
            "AssignmentRules",
            "AutoResponseRule",
            "AutoResponseRules",
            "EscalationRule",
            "EscalationRules",
            "MatchingRule",
            "MatchingRules",
            "DuplicateRule",
        ],
    },
    TypeCategory {
        name: "experience",
        feature: "experience",
        module_path: "metadata/experience.rs",
        description: "Experience Cloud, communities, and networks",
        patterns: &["Experience*", "Community*", "Network*", "Site*"],
        explicit_types: &[
            "ExperienceBundle",
            "ExperienceContainer",
            "ExperiencePropertyTypeBundle",
            "ExperienceResources",
            "CommunityTemplateDefinition",
            "CommunityThemeDefinition",
            "Network",
            "NetworkBranding",
            "CustomSite",
            "SiteDotCom",
        ],
    },
    TypeCategory {
        name: "reports",
        feature: "reports",
        module_path: "metadata/reports.rs",
        description: "Reports, dashboards, and analytics",
        patterns: &["Report*", "Dashboard*"],
        explicit_types: &[
            "Report",
            "ReportAggregate",
            "ReportBucketField",
            "ReportBucketFieldValue",
            "ReportChart",
            "ReportColorRange",
            "ReportColumn",
            "ReportCrossFilter",
            "ReportDataCategoryFilter",
            "ReportFilter",
            "ReportFilterItem",
            "ReportFormattingRule",
            "ReportFormattingRuleValue",
            "ReportGrouping",
            "ReportHistoricalSelector",
            "ReportParam",
            "ReportTimeFrameFilter",
            "ReportType",
            "ReportTypeColumn",
            "ReportTypeSectionTranslation",
            "Dashboard",
            "DashboardComponent",
            "DashboardComponentColumn",
            "DashboardComponentSection",
            "DashboardComponentSortInfo",
            "DashboardFilter",
            "DashboardFilterColumn",
            "DashboardFilterOption",
            "DashboardFlexTableComponentProperties",
            "DashboardGridComponent",
            "DashboardGridLayout",
            "DashboardMobileSettings",
            "DashboardTableColumn",
        ],
    },
    // ==================== PACKAGING ====================
    TypeCategory {
        name: "packaging",
        feature: "packaging",
        module_path: "packaging/packages.rs",
        description: "1GP and 2GP package types",
        patterns: &["Package*", "Subscriber*", "Installed*"],
        explicit_types: &[
            "Package",
            "PackageVersion",
            "Package2",
            "Package2Version",
            "InstalledPackage",
            "SubscriberPackage",
            "SubscriberPackageVersion",
            "PackageTypeMembers",
            "ProfileObjectPermissions",
        ],
    },
    // ==================== COMMON ENUMS ====================
    TypeCategory {
        name: "common_enums",
        feature: "common",
        module_path: "common/enums.rs",
        description: "Common enums used across multiple metadata types",
        patterns: &[],
        explicit_types: &[
            "FieldType",
            "SharingModel",
            "DeploymentStatus",
            "DeployStatus",
            "Gender",
            "StartsWith",
            "PlatformCacheType",
            "TreatBlanksAs",
            "EncryptedFieldMaskChar",
            "EncryptedFieldMaskType",
            "DeleteConstraint",
            "MonitoredEvents",
            "RetrieveStatus",
            "ManageableState",
            "ActionEmailRecipientTypes",
            "ActionEmailSenderType",
            "ActionTaskAssignedToTypes",
            "APIAccessLevel",
            "Article",
            "ChartBackgroundDirection",
            "ChartLegendPosition",
            "ChartPosition",
            "ChartRangeType",
            "ChartTheme",
            "ChartType",
            "ChartUnits",
            "ComponentInstancePropertyTypeEnum",
            "Encoding",
            "FieldManageability",
            "FieldUpdateOperation",
            "FilterOperation",
            "FilterScope",
            "ForecastCategories",
            "Language",
            "LayoutHeader",
            "LayoutSectionStyle",
            "Possessive",
            "ReportAggregateDatatype",
            "ReportBucketDateGranularity",
            "ReportChartSize",
            "ReportFormat",
            "ReportSortType",
            "SearchItemActionType",
            "SetupObjectVisibility",
            "SortOrder",
            "SummaryLayoutStyle",
            "TabVisibility",
            "UiBehavior",
            "WebLinkAvailability",
            "WebLinkDisplayType",
            "WebLinkPosition",
            "WebLinkType",
            "WebLinkWindowType",
            "WorkflowTimeUnits",
        ],
    },
    // ==================== EMAIL ====================
    TypeCategory {
        name: "email",
        feature: "email",
        module_path: "metadata/email.rs",
        description: "Email templates and configuration",
        patterns: &["Email*", "Letterhead*"],
        explicit_types: &[
            "EmailTemplate",
            "EmailServicesAddress",
            "EmailServicesFunction",
            "EmailToCaseRoutingAddress",
            "EmailToCaseSettings",
            "Letterhead",
            "LetterheadLine",
            "LetterheadHeaderFooter",
            "EnhancedLetterhead",
            "EnhancedLetterheadHeader",
            "EnhancedLetterheadFooter",
            "OrgWideEmailAddress",
        ],
    },
    // ==================== BOTS / EINSTEIN ====================
    TypeCategory {
        name: "bots",
        feature: "bots",
        module_path: "metadata/bots.rs",
        description: "Einstein Bots and conversational AI",
        patterns: &["Bot*"],
        explicit_types: &[
            "Bot",
            "BotVersion",
            "BotBlock",
            "BotBlockVersion",
            "BotDialog",
            "BotDialogGroup",
            "BotIntent",
            "BotInvocation",
            "BotMessage",
            "BotNavigation",
            "BotNavigationLink",
            "BotQuickReplyOption",
            "BotSlot",
            "BotStep",
            "BotStepCondition",
            "BotTemplate",
            "BotVariableOperation",
            "EinsteinAgent",
        ],
    },
    // ==================== INTEGRATION ====================
    TypeCategory {
        name: "integration",
        feature: "integration",
        module_path: "metadata/integration.rs",
        description: "Connected apps, external services, and integrations",
        patterns: &["ConnectedApp*", "ExternalService*", "NamedCredential*"],
        explicit_types: &[
            "ConnectedApp",
            "ConnectedAppOauthConfig",
            "ConnectedAppOauthPolicy",
            "ConnectedAppSamlConfig",
            "ExternalDataSource",
            "ExternalServiceRegistration",
            "NamedCredential",
            "ExternalCredential",
            "PlatformEventChannel",
            "PlatformEventChannelMember",
            "EventRelayConfig",
            "EventType",
            "RemoteSiteSetting",
            "CspTrustedSite",
        ],
    },
    // ==================== NEW CATEGORIES (Phase 1 Expansion) ====================

    // Analytics / Wave / CRM Analytics
    TypeCategory {
        name: "analytics",
        feature: "analytics",
        module_path: "metadata/analytics.rs",
        description: "Wave Analytics, CRM Analytics, Tableau CRM types",
        patterns: &[
            "Wave*",
            "Xmd*",
            "Analytics*",
            "Lens*",
            "Dataset*",
            "Dataflow*",
        ],
        explicit_types: &[
            "WaveApplication",
            "WaveDataset",
            "WaveTemplateBundle",
            "WaveXmd",
            "WaveXmdDate",
            "WaveXmdDimension",
            "WaveXmdDimensionCustomAction",
            "WaveXmdDimensionMember",
            "WaveXmdDimensionSalesforceAction",
            "WaveXmdFormattingBin",
            "WaveXmdFormattingPredicate",
            "WaveXmdFormattingProperty",
            "WaveXmdMeasure",
            "WaveXmdOrganization",
            "WaveXmdRecordDisplayLookup",
            "WaveAnalyticAssetCollection",
            "WaveAnalyticAssetCollectionItem",
            "WaveTemplateExternalDataMetadata",
            "AnalyticSnapshot",
            "AnalyticSnapshotMapping",
        ],
    },
    // OmniStudio
    TypeCategory {
        name: "omnistudio",
        feature: "omnistudio",
        module_path: "metadata/omnistudio.rs",
        description: "OmniStudio, FlexiPages, OmniScripts, and DataRaptors",
        patterns: &["Omni*", "Flexi*", "IntegrationProcedure*", "DataRaptor*"],
        explicit_types: &[
            "OmniScript",
            "OmniDataTransform",
            "OmniDataTransformItem",
            "OmniProcess",
            "OmniProcessType",
            "OmniSupervisorConfig",
            "OmniSupervisorConfigSkill",
            "OmniSupervisorConfigTab",
            "OmniSupervisorActionTab",
            "OmniSupervisorActionName",
            "OmniSuperSkillVisibilityType",
            "OmniSupervisorTabType",
            "OmniTrackingGroupType",
            "OmniTrackingComponentDef",
            "OmniExtTrackingDef",
            "OmniDataTransformInputType",
            "OmniAnalyticsComponentType",
            "OmniUiCardType",
            "OmniAssessmentTaskMetadata",
            "FlexiPage",
            "FlexiPageEvent",
            "FlexiPageEventTargetProperty",
            "FlexiPageEventSourceProperty",
            "FlexiPageType",
            "FlexiPageRegionType",
            "FlexiPageRegionMode",
            "FlexipageDataSourceModeEnum",
            "FlexipageDataSourceTypeEnum",
            "FlexipageEventSourceTypeEnum",
            "FlexipageEventTargetTypeEnum",
            "FlexipageSchemaPropType",
        ],
    },
    // AI / ML / Einstein
    TypeCategory {
        name: "ai_ml",
        feature: "ai",
        module_path: "metadata/ai.rs",
        description: "AI, Machine Learning, Einstein, Prompt Templates",
        patterns: &[
            "Ml*",
            "Ai*",
            "GenAi*",
            "Prediction*",
            "Prompt*",
            "Einstein*",
        ],
        explicit_types: &[
            "MlAIModelType",
            "MlGenerativeModelCapability",
            "MlGenerativeModelType",
            "MlModelSourceType",
            "MlSlotClassExtractionType",
            "MlModelConnectorType",
            "MlSlotClassDataType",
            "MlInferenceFormat",
            "MlModelPredictionType",
            "MlModelType",
            "MlAIModelAlgorithmType",
            "MlModelCapability",
            "MlModelEndpointType",
            "MlParameterType",
            "MlRuntimeType",
            "MlParameterSubtype",
            "MlModelDeployStatus",
            "MlModelOutput",
            "MlIntentUtterance",
            "MlModelOutputEndpoint",
            "MlSlotClassValue",
            "MlParameterOverride",
            "MlModelInput",
            "Prompt",
            "PromptDisplayPosition",
            "PromptElementRelativePosition",
            "PromptThemeColor",
            "PromptUserAccess",
            "PromptImageLocation",
            "PromptExperience",
            "PromptDisplayType",
            "PromptThemeSaturation",
            "PromptUserProfileAccess",
            "PromptVersionTranslation",
            "PredictionPlatform",
            "AiEvaluationTestCaseCritParam",
            "AiEvaluationExpectation",
            "AiEvalCopilotTestCaseConv",
            "AiEvaluationAgentTestCaseInput",
            "AiEvaluationDefinition",
            "EinsteinAgent",
            "EinsteinGptSettings",
        ],
    },
    // Decision Tables / Business Rules Engine
    TypeCategory {
        name: "decision_tables",
        feature: "decisions",
        module_path: "metadata/decisions.rs",
        description: "Decision Tables, Expression Sets, Business Rules",
        patterns: &[
            "Decision*",
            "Expression*",
            "Exps*",
            "Calculation*",
            "Matrix*",
        ],
        explicit_types: &[
            "DecisionTable",
            "DecisionTableSortType",
            "DecisionMatrixColumnType",
            "DecisionTableConditionType",
            "DecisionMatrixDataType",
            "DecisionTableDataSourceType",
            "DecisionMatrixDefStatus",
            "DecisionTableDownloadStatus",
            "DecisionTableHitPolicy",
            "DecisionMatrixType",
            "DecisionTableRefreshStatus",
            "DecisionTableType",
            "DecisionTableParameterType",
            "DecisionTableCollectOperator",
            "DecisionTableUploadStatus",
            "DecisionTableExecutionType",
            "DecisionTableOperator",
            "DecisionTableStatus",
            "DecisionTblDatasetParameter",
            "DecisionTableSourceCriteria",
            "ExpressionType",
            "ExpressionSetStepType",
            "ExpsSetConditionOperator",
            "ExpsSetStatus",
            "ExpsSetProcessType",
            "ExpsSetVariableLookupType",
            "ExpsSetStepType",
            "ExpsSetInterfaceSourceType",
            "ExpsSetDataType",
            "ExpsSetValueType",
            "ExpsSetVariableType",
            "ExpsSetAggregationFunction",
            "ExpsSetUsageSubtype",
            "ExpsSetExecutionScale",
            "ExpsSetObjectDataType",
            "ExpressionSetStep",
            "ExpressionSetElementParameter",
            "ExpressionSetSubExpression",
            "CalculatedInsightCreationType",
            "CalculatedInsightDefinitionType",
            "CalculationFrequency",
        ],
    },
    // Data Cloud / CDP
    TypeCategory {
        name: "data_cloud",
        feature: "datacloud",
        module_path: "metadata/datacloud.rs",
        description: "Data Cloud, CDP, Data Sources, Data Connections",
        patterns: &["Data*", "Cdp*", "Dataspace*"],
        explicit_types: &[
            "DataPipelineType",
            "DataConnectorCapability",
            "DatasetColumnDataType",
            "DataCategoryFilterOperation",
            "DataImportDataExtractMethods",
            "DataModelType",
            "DataConnectorType",
            "DataConnectorFeature",
            "DataConnectionStatus",
            "DatatableDataType",
            "DataSourceType",
            "DataObjectType",
            "DataConnectorDataType",
            "DataConnectorReleaseLevel",
            "DataImportRefreshFrequency",
            "DataImportRefreshMode",
            "DataPlatformDataSet",
            "DataSourceField",
            "DatasetImportRequest",
            "DataConnectorAttributeOpt",
            "DataSourceBundleDefinition",
            "DataConnectorS3",
            "DataPlatform",
            "DataSourceObject",
            "DataPackageKitDefinition",
            "DataSource",
            "DataConnectionParamTmpl",
            "DataConnectorTranslation",
            "DataKitObjectTemplate",
            "DataModelTaxonomy",
            "DataObjectCategory",
            "DataspaceScope",
            "DataCleanRoomProvider",
            "DataKitObjectDependency",
            "DataObjectSearchIndexConf",
            "DataObjectBuildOrgTemplate",
            "DataCategory",
            "DataspaceScopeSchemaAccess",
            "DataSrcDataModelFieldMap",
        ],
    },
    // Service Cloud
    TypeCategory {
        name: "service_cloud",
        feature: "servicecloud",
        module_path: "metadata/servicecloud.rs",
        description: "Service Cloud, Cases, Knowledge, Entitlements, Embedded Service",
        patterns: &[
            "Embedded*",
            "Case*",
            "Knowledge*",
            "Entitlement*",
            "LiveAgent*",
            "LiveChat*",
        ],
        explicit_types: &[
            "EmbeddedServiceResourceType",
            "EmbeddedServiceDeploymentFeature",
            "EmbeddedServiceCustomComponentType",
            "EmbeddedServiceComponentBundleType",
            "EmbeddedServiceScenario",
            "EmbeddedServiceFlowType",
            "EmbeddedServiceLayoutType",
            "EmbeddedServiceChannelType",
            "EmbeddedServiceFontSize",
            "EmbeddedServiceFormDisplayContext",
            "EmbeddedServiceFeature",
            "EmbeddedServiceClientVersion",
            "EmbeddedServiceAuthMethod",
            "EmbeddedServiceAuthModeType",
            "EmbeddedMsgQueueLimitType",
            "EmbeddedServiceDeploymentType",
            "EmbeddedServiceQuickActionType",
            "EmbeddedServiceFormFieldType",
            "EmbeddedServiceLabelKey",
            "EmbeddedServiceFlowConfig",
            "EmbeddedServiceCustomization",
            "EmbeddedServiceLiveAgent",
            "EmbeddedServiceFormField",
            "CaseSubjectOption",
            "CaseSubjectParticle",
            "CaseType",
            "CaseSubjectParticleType",
            "KnowledgeLanguageLookupValueType",
            "KnowledgeWorkflowAction",
            "KnowledgeCaseEditor",
            "KnowledgeWorkOrderField",
            "ServiceMgmtKnwlgArtclConfig",
            "LiveChatButtonType",
            "LiveChatButtonRoutingType",
            "LiveChatButtonInviteStartPosition",
            "LiveChatButtonPresentation",
            "LiveChatButtonInviteEndPosition",
            "LiveChatButtonSkills",
            "LiveAgentConfig",
        ],
    },
    // Service Catalog
    TypeCategory {
        name: "service_catalog",
        feature: "servicecatalog",
        module_path: "metadata/servicecatalog.rs",
        description: "Service Catalog items and configurations",
        patterns: &["Svc*", "Service*"],
        explicit_types: &[
            "SvcCatalogItemAttrDataType",
            "SvcCatalogItemUsageType",
            "SvcCtlgItemDpndProcType",
            "SvcCatalogItemDependencyType",
            "SvcCatalogItemAttrType",
            "SvcCtlgItemAttrAttributeType",
            "SvcCatalogFilterCondition",
            "SvcCatalogItemAttrDetail",
            "ServiceAISetupFieldType",
            "ServicePlanSourceType",
            "ServiceAISetupDefStatus",
            "ServiceProcessItemGroup",
        ],
    },
    // Messaging
    TypeCategory {
        name: "messaging",
        feature: "messaging",
        module_path: "metadata/messaging.rs",
        description: "Messaging, Chat, Conversations, Chatter",
        patterns: &["Messaging*", "Conv*", "Chat*", "Chatter*"],
        explicit_types: &[
            "ConvIntelligenceActionType",
            "ConvMsgExternalTemplateVersionType",
            "MessagingChannelParameterType",
            "ConvIntelligenceService",
            "MessagingChannelConsentType",
            "MessagingKeywordType",
            "ConvParticipantRole",
            "MessagingChannelUsageDeploymentType",
            "ConvMsgExternalErrorCode",
            "ConvIntelligenceOperator",
            "MessagingAuthorizationType",
            "ConvIntelligenceType",
            "MessagingChannelStandardParameterType",
            "ConvMsgExternalRejectionReason",
            "MessagingChannelTargetLookupValueType",
            "MessagingSessionHandlerType",
            "ChatterExtensionType",
            "ConvMsgExternalTemplateVersionStatus",
            "MessagingAutoResponseType",
            "MessagingChannelType",
            "ConvDefBlockVersionStatus",
            "ConvMsgExternalTemplateVersion",
            "MessagingChannelStandardParameter",
            "MessagingChannelActionParameterMapping",
            "MessagingAutoResponse",
            "ConvReasonReportDefinition",
            "MessagingChannelParameterValueMapping",
            "ConvIntelligenceSignalSubRule",
            "ChatterExtension",
            "ConvReasonReportSegmentDef",
            "MessagingChannel",
            "MessagingKeyword",
            "MessagingAuthorization",
            "MessagingChannelCustomParameter",
            "ConvIntelligenceSignalRule",
            "MessagingChannelUsage",
            "ChatterAnswersReputationLevel",
            "ConversationDefinition",
            "ConversationMessageDefinition",
        ],
    },
    // Loyalty Cloud
    TypeCategory {
        name: "loyalty",
        feature: "loyalty",
        module_path: "metadata/loyalty.rs",
        description: "Loyalty Cloud, Programs, Benefits, Vouchers",
        patterns: &["Loyalty*", "Benefit*", "Voucher*"],
        explicit_types: &[
            "LoyaltyPgmProcParmDataType",
            "LoyaltyPgmProcRuleStatus",
            "LoyaltyPgmProcActionType",
            "BenefitActionDataType",
            "LoyaltyPgmProcCrudActType",
            "LoyaltyPgmProcStatus",
            "LoyaltyPgmProcActParamType",
            "LoyaltyPgmProcCondOperator",
            "LoyaltyPgmProcCondType",
            "LoyaltyPgmProcParmType",
            "LoyaltyPgmProcRuleType",
            "LoyaltyPgmProcExecutionType",
            "LoyaltyPgmProcActParamOper",
            "LoyaltyProgramProcessRuleStepMapping",
            "LoyaltyProgramSetup",
            "BenefitActionParameterValue",
            "LoyaltyProgramProcessAction",
            "LoyaltyProgramProcessActionParameter",
            "LoyaltyProgramProcessParameter",
            "LoyaltyProgramProcessCondition",
            "LoyaltyProgramProcessConditionFilterCriteria",
            "LoyaltyProgramProcessRule",
            "LoyaltyProgramProcess",
            "BenefitActionParameter",
            "BenefitAction",
        ],
    },
    // Identity / Authentication
    TypeCategory {
        name: "identity",
        feature: "identity",
        module_path: "metadata/identity.rs",
        description: "Identity, SAML, OAuth, Authentication, SSO",
        patterns: &["Identity*", "Saml*", "Sso*", "Auth*", "Login*", "Oauth*"],
        explicit_types: &[
            "SamlSigningAlgoType",
            "IdentityProviderAuthProtocol",
            "SamlSubjectType",
            "SamlIdentityLocationType",
            "IdentityVerificationDataSourceType",
            "LoginFlowType",
            "SamlEncryptionType",
            "SamlNameIdFormatType",
            "AuthProviderType",
            "SamlType",
            "SamlIdentityType",
            "IdentityVerificationProcFldDataSourceType",
            "IdentityVerificationProcFldFieldType",
            "AuthenticationProtocol",
            "IdentityVerificationSearchType",
            "IdentityProviderAuthFlow",
            "AuthType",
            "AuthoringMode",
            "SamlSpSLOBinding",
            "SamlIdpSLOBinding",
            "IdentityVerificationProcFldFieldDataType",
            "SamlInitiationMethod",
            "IdentityVerificationSearchLayoutType",
            "OauthTokenExchHandlerApp",
            "OauthCustomScope",
            "LoginFlow",
            "OauthCustomScopeApp",
            "IdentityVerificationFieldTranslation",
            "AuthProvider",
            "IdentityVerificationProcDef",
            "OauthTokenExchangeHandler",
            "SamlSsoConfig",
            "IdentityVerificationProcDtl",
            "IdentityVerificationProcFld",
            "AuthProvParamFwdAllowlist",
        ],
    },
    // Scheduling / Field Service
    TypeCategory {
        name: "scheduling",
        feature: "scheduling",
        module_path: "metadata/scheduling.rs",
        description: "Scheduling, Field Service, Appointments, Territories",
        patterns: &[
            "Scheduling*",
            "Appointment*",
            "Shift*",
            "Territory*",
            "FieldService*",
        ],
        explicit_types: &[
            "SchedulingObjectiveType",
            "SchedulingRuleType",
            "ShiftSegmentTypeCategory",
            "SchedulingMode",
            "SchedulingParameterKey",
            "SchedulingCategory",
            "AppointmentAssignmentPolicy",
            "Territory2Type",
            "Territory2AccessLevel",
            "FieldServiceMobileConfig",
            "Territory2Model",
            "AppointmentSchedulingPolicy",
            "SchedulingRuleParameter",
            "Territory2RuleAssociation",
            "SchedulingRule",
            "Territory2Rule",
            "Territory2RuleItem",
            "Territory2",
            "SchedulingObjective",
            "SchedulingObjectiveParameter",
            "ShiftSegmentType",
        ],
    },
    // Batch Processing
    TypeCategory {
        name: "batch",
        feature: "batch",
        module_path: "metadata/batch.rs",
        description: "Batch Jobs, Async Processing, Queues",
        patterns: &["Batch*", "Async*", "Queue*"],
        explicit_types: &[
            "BatchCalcJobDataType",
            "BatchCalcJobAggregateFunction",
            "BatchJobDefinitionStatus",
            "BatchCalcJobDefRunMode",
            "BatchCalcJobFrcstModel",
            "AsyncRequestState",
            "BatchCalcJobFrcstAccuracy",
            "BatchCalcJobParameterDataType",
            "BatchCalcJobCSVDelimiter",
            "BatchCalcJobWritebackOpn",
            "BatchCalcJobFilterOperator",
            "BatchCalcJobFrcstPeriodType",
            "BatchInputSourceType",
            "BatchCalcJobFileSource",
            "BatchCalcJobWritebackKeyType",
            "BatchCalcJobDatasourceType",
            "BatchCalcJobTransformType",
            "BatchCalcJobSourceJoinType",
            "BatchCalcProcessType",
            "BatchCalcJobFrcstSeasonality",
            "BatchCalcJobOrderType",
            "BatchCalcJobWritebackType",
            "AsyncResult",
            "BatchCalcJobTransformAddedField",
            "QueueMembers",
            "QueueRoutingConfig",
            "BatchProcessJobDefinition",
            "BatchCalcJobWritebackMapping",
            "BatchDataSrcFilterCriteria",
            "BatchCalcJobCustomNodeParameter",
            "QueueSobject",
            "BatchCalcJobFrcstGrpFld",
            "BatchCalcJobOrderByField",
            "BatchCalcJobJoinKey",
            "BatchCalcJobDatasourceField",
            "BatchCalcJobAtomicWritebackRelationship",
            "BatchCalcJobFilterCriteria",
            "Queue",
            "BatchCalcJobAggregateField",
            "BatchDataSourceOrderField",
        ],
    },
    // Quick Actions
    TypeCategory {
        name: "quick_actions",
        feature: "quickactions",
        module_path: "metadata/quickactions.rs",
        description: "Quick Actions, Global Actions, Action Links",
        patterns: &[
            "QuickAction*",
            "ActionLink*",
            "ActionPlan*",
            "ActionLauncher*",
        ],
        explicit_types: &[
            "QuickActionType",
            "QuickActionParameterType",
            "QuickActionLabel",
            "QuickActionSendEmailOptions",
            "QuickActionLayoutItem",
            "QuickAction",
            "QuickActionLayout",
            "QuickActionParameters",
            "QuickActionTranslation",
            "QuickActionLayoutColumn",
            "QuickActionParametersTranslation",
            "ActionLinkHttpMethod",
            "ActionLinkUserVisibility",
            "ActionLinkType",
            "ActionLinkExecutionsAllowed",
            "ActionLinkGroupTemplate",
            "ActionLinkTemplate",
            "ActionPlanTemplateType",
            "ActionPlanTemplate",
            "ActionPlanTemplateItemValue",
            "ActionPlanTemplateItemDependency",
            "ActionPlanTemplateItem",
            "ActionLauncherItemDef",
            "GlobalPicklistValue",
            "GlobalPicklist",
            "GlobalQuickActionTranslation",
        ],
    },
    // Custom Metadata / Records
    TypeCategory {
        name: "custom_metadata",
        feature: "custommetadata",
        module_path: "metadata/custommetadata.rs",
        description: "Custom Metadata, Custom Labels, Custom Tabs",
        patterns: &[
            "CustomLabel*",
            "CustomTab*",
            "CustomMetadata*",
            "CustomPermission*",
            "CustomNotification*",
        ],
        explicit_types: &[
            "CustomSettingsType",
            "CustomChannelConnectedAppType",
            "CustomLabel",
            "CustomNotificationActionGroup",
            "CustomDataType",
            "CustomHelpMenuSection",
            "CustomConsoleComponents",
            "CustomPermission",
            "CustomMetadata",
            "CustomDataTypeComponent",
            "CustomApplicationComponent",
            "CustomTab",
            "CustomLabelTranslation",
            "CustomHttpHeader",
            "CustomPermissionDependencyRequired",
            "CustomMetadataValue",
            "CustomPageWebLink",
            "CustomNotificationType",
            "CustomPageWebLinkTranslation",
        ],
    },
    // Record Actions / Aggregation
    TypeCategory {
        name: "record_actions",
        feature: "recordactions",
        module_path: "metadata/recordactions.rs",
        description: "Record Actions, Record Aggregations, Record Alerts",
        patterns: &["Record*"],
        explicit_types: &[
            "RecordActionType",
            "RecordAggregationDefinitionStatus",
            "RecordTriggerType",
            "RecordEditabilityType",
            "RecordAggregationObjectFilterOperator",
            "RecordAlertDataSourceType",
            "RecordAggregationJoinConditionType",
            "RecordAggregationDefinitionAggregationType",
            "RecordActionDeploymentContext",
            "RecordActionDeployment",
            "RecordAggregationDefinition",
            "RecordAlertTemplate",
            "RecordActionSelectableItem",
            "RecordTypeTranslation",
            "RecordTypePicklistValue",
            "RecordAggregationObjectFilter",
            "RecordAggregationObject",
            "RecordAggregationJoinCondition",
            "RecordAlertDataSource",
            "RecordActionDeploymentChannel",
            "RecordActionDefaultItem",
        ],
    },
    // External Client Apps
    TypeCategory {
        name: "external_apps",
        feature: "externalapps",
        module_path: "metadata/externalapps.rs",
        description: "External Client Apps, Mobile Apps, Canvas Apps",
        patterns: &["Extl*", "External*"],
        explicit_types: &[
            "ExtlClntAppDistState",
            "ExtlClntAppStartPage",
            "ExtlClntAppSamlSubjectType",
            "ExtlIdentityProviderParmType",
            "ExtlClntAppSamlBindingType",
            "ExtlClntAppSamlSignAlgoType",
            "ExtlClntAppNameIdFormatType",
            "ExtlClntAppManagedType",
            "ExtlClntAppSamlEncryptType",
            "ExtlClntAppSamlConfigurablePoliciesAttribute",
            "ExtlClntAppOauthSettingsAttribute",
            "ExtlClntAppPushConfigurablePolicies",
            "ExtlClntAppOauthPoliciesAttribute",
            "ExtlClntAppOauthConfigurablePolicies",
            "ExtlClntAppNotificationType",
            "ExtlClntAppApplePushConfig",
            "ExtlClntAppConfigurablePolicies",
            "ExtlClntAppMobileConfigurablePolicies",
            "ExtlClntAppOauthIpRange",
            "ExtlClntAppSampleConfigurablePolicies",
            "ExtlClntAppSamlConfigurablePolicies",
            "ExtlClntAppAndroidPushConfig",
            "ExtlClntAppCanvasStngs",
        ],
    },
    // Discovery / Einstein Discovery
    TypeCategory {
        name: "discovery",
        feature: "discovery",
        module_path: "metadata/discovery.rs",
        description: "Einstein Discovery, Predictive Analytics",
        patterns: &["Discovery*"],
        explicit_types: &[
            "DiscoveryFieldMapSourceType",
            "DiscoveryFilterFieldType",
            "DiscoveryFilterValueType",
            "DiscoveryModelFieldType",
            "DiscoveryStorySourceType",
            "DiscoveryPredictionType",
            "DiscoveryAlgorithmType",
            "DiscoveryAIModelTransformationType",
            "DiscoveryStoryOutcomeType",
            "DiscoveryFilterOperator",
            "DiscoveryOutcomeGoal",
            "DiscoveryAIModelStatus",
            "DiscoveryStoryAutopilotStatus",
            "DiscoveryModelRuntimeType",
            "DiscoveryModelSourceType",
            "DiscoveryPushbackType",
            "DiscoveryStoryOutcomeGoal",
            "DiscoveryModelField",
            "DiscoveryGoalOutcome",
            "DiscoveryFilterValue",
            "DiscoveryFilter",
            "DiscoveryDeployedModel",
            "DiscoveryStoryOutcome",
            "DiscoveryPrescribableField",
            "DiscoveryFieldMap",
            "DiscoveryModelTransform",
            "DiscoveryCustomPrescribableFieldDefinition",
            "DiscoveryGoal",
            "DiscoveryModelCard",
        ],
    },
    // Marketing Cloud
    TypeCategory {
        name: "marketing",
        feature: "marketing",
        module_path: "metadata/marketing.rs",
        description: "Marketing Cloud Connect, Campaigns, Audiences",
        patterns: &["Mkt*", "Market*", "Campaign*", "Audience*"],
        explicit_types: &[
            "MktDataModelFieldUsageTag",
            "MarketAudienceStatus",
            "MktDataConnectionStatus",
            "MktDataConnectionMethod",
            "MarketSegmentType",
            "MktDataConnection",
            "MktDataModelFieldAttributes",
            "MktDataTranField",
            "MktDataLakeAttributes",
            "MarketingAppExtAction",
            "MktDataLakeFieldAttributes",
            "MarketingAppExtension",
            "MktDatalakeSrcKeyQualifier",
            "CampaignInfluenceModel",
            "MktDataConnectionSrcParam",
            "MktDataTranObject",
            "MarketSegmentDefinition",
            "MarketingAppExtActivity",
            "MktDataModelAttributes",
            "MktDataConnectionParam",
            "MarketAudienceDefinition",
            "MktDataConnectionCred",
            "MarketAudienceField",
            "CampaignTemplateDefinition",
            "MktCalcInsightObjectDef",
        ],
    },
    // Search
    TypeCategory {
        name: "search",
        feature: "search",
        module_path: "metadata/search.rs",
        description: "Search customization and configuration",
        patterns: &["Search*"],
        explicit_types: &[
            "SearchCriteriaConfigurationResultDisplayFormat",
            "SearchCriteriaConfigurationFilterType",
            "SearchCriteriaConfigurationConfigurationType",
            "SearchResultActionScope",
            "SearchResultActionType",
            "SearchableObjDataSyncInfo",
            "SearchCustomizationObjectOverride",
            "SearchCriteriaConfiguration",
            "SearchResultActionConfig",
            "SearchCustomizationRule",
            "SearchOrgWideObjectConfig",
            "SearchSettingsByObject",
            "SearchCustomizationExplicitFilter",
            "SearchCustomizationFieldOverride",
            "SearchCustomizationRuleValue",
            "SearchCustomization",
            "SearchOrgWideFieldConfig",
        ],
    },
    // Platform Events / Notifications
    TypeCategory {
        name: "platform_events",
        feature: "platformevents",
        module_path: "metadata/platformevents.rs",
        description: "Platform Events, Event Subscriptions, Notifications",
        patterns: &["PlatformEvent*", "Event*", "Notification*", "Ntfcn*"],
        explicit_types: &[
            "PlatformEventChannelEventType",
            "PlatformEventType",
            "PlatformSchemaContentType",
            "PlatformEventChannelType",
            "PlatformActionType",
            "PlatformActionGroupCategory",
            "EventRelayUsageType",
            "EventSubscriptionAdminState",
            "EventSubscriptionReplayPreset",
            "EventRelayAdminState",
            "PlatformActionListContext",
            "PlatformEventPublishBehavior",
            "NotificationActionType",
            "EventDeliveryType",
            "NtfcnChannelCont",
            "PlatformLicenseDefinition",
            "EventLogObject",
            "NotificationTypeConfig",
            "PlatformCachePartitionType",
            "NtfcnChannelActionDef",
            "PlatformEventSubscriberConfig",
            "EventSubscription",
            "EventParameterMap",
            "NtfcnDefinition",
            "NtfcnCondition",
            "NotificationChannels",
            "NtfcnChannelRec",
            "PlatformCachePartition",
            "NtfcnCriteria",
            "NtfcnChannelDef",
        ],
    },
    // User Access / Policies
    TypeCategory {
        name: "user_access",
        feature: "useraccess",
        module_path: "metadata/useraccess.rs",
        description: "User Access Policies, User Provisioning, User Criteria",
        patterns: &["User*"],
        explicit_types: &[
            "UserAccessPolicyFilterOperation",
            "UserAccessPolicyTriggerType",
            "UserDateInterval",
            "UserDateGranularity",
            "UserAccessPolicyActionType",
            "UserAccessPolicyFilterTargetType",
            "UserAccessPolicyActionTargetType",
            "UserAccessPolicyStatus",
            "UserAccessPolicyAction",
            "UserProvisioningConfig",
            "UserAccessPolicy",
            "UserLicenseDefinition",
            "Users",
            "UserCriteria",
            "UserAccessPolicyFilter",
        ],
    },
    // Context / Activation Platform
    TypeCategory {
        name: "activation",
        feature: "activation",
        module_path: "metadata/activation.rs",
        description: "Context Definitions, Activation Platform, Enablement",
        patterns: &["Context*", "Activation*", "Enablement*"],
        explicit_types: &[
            "ActivationFlowType",
            "ContextAttributeFieldType",
            "ActivationPlatformFieldDataType",
            "ActivationPlatformRefreshFrequency",
            "ActivationPlatformFileOutputFormat",
            "ContextRuleStatus",
            "ActivationPlatformIdentifierHashMethod",
            "ActivationPlatformFileOutputGrouping",
            "ContextRuleUsageType",
            "ActivationPlatformIdentifierType",
            "ActivationPlatformProcessingType",
            "ActivationPlatformType",
            "EnablementFilterOperator",
            "ContextUseCaseType",
            "ActivationPlatformCreationType",
            "ContextMappingConfigUsageType",
            "ContextAttributeDataType",
            "ActivationPlatformRefreshMode",
            "ActivationPlatformConnectorType",
            "ContextMappingType",
            "ContextMappingIntentType",
            "ActivationPlatformPeriodicFullRefresh",
            "EnablementAggregationType",
            "ContextNode",
            "ContextAttributeMapping",
            "ContextMapping",
            "EnablementProgramTaskExternalContent",
            "ContextTag",
            "ActivationPlatformActvAttr",
            "ContextUseCaseMapping",
            "ContextDefinition",
            "ActivationPlatform",
        ],
    },
    // Industries (Health, Insurance, Automotive, Manufacturing)
    TypeCategory {
        name: "industries",
        feature: "industries",
        module_path: "metadata/industries.rs",
        description: "Industry Cloud types (Health, Insurance, Manufacturing, Automotive)",
        patterns: &[
            "Care*",
            "Health*",
            "Clinical*",
            "Insurance*",
            "Ins*",
            "Mfg*",
            "Manufacturing*",
            "Vehicle*",
            "Automotive*",
        ],
        explicit_types: &[
            "MfgProgramTransformationType",
            "VehicleAssetType",
            "CareLimitTypeMetricType",
            "CareProviderAfflType",
            "MfgProgramTemplateStatus",
            "MfgProgramTemplate",
            "CareLimitType",
            "VehicleAssetEmssnSrcCnfg",
            "MfgProgramTemplateItem",
            "CareProviderAfflRoleConfig",
            "CareProviderSearchConfig",
            "CareRequestRecords",
            "CareSystemFieldMapping",
            "CareRequestConfiguration",
        ],
    },
    // Commerce
    TypeCategory {
        name: "commerce",
        feature: "commerce",
        module_path: "metadata/commerce.rs",
        description: "Commerce Cloud, Web Stores, Products, Pricing",
        patterns: &[
            "Commerce*",
            "WebStore*",
            "Product*",
            "PriceBook*",
            "Cart*",
            "Checkout*",
            "Order*",
            "Payment*",
            "Promotion*",
        ],
        explicit_types: &[
            "ProductGrouping",
            "WebStoreType",
            "ProductFamilyUsageType",
            "OrderLifeCycleType",
            "WebStoreBundle",
            "ProductAttributeSetItem",
            "WebStoreTemplate",
            "PaymentGatewayProvider",
            "ProductAttributeSet",
            "ProductSpecificationType",
            "ProductSpecificationRecType",
            "ProductFamilyUsage",
            "ProductAttrDisplayConfig",
        ],
    },
    // Field Mappings
    TypeCategory {
        name: "field_mappings",
        feature: "fieldmappings",
        module_path: "metadata/fieldmappings.rs",
        description: "Field Mappings, Field Restrictions, Field Configurations",
        patterns: &["FieldMapping*", "FieldRestriction*", "FieldSource*"],
        explicit_types: &[
            "FieldMappingClient",
            "FieldSrcTrgtRelationshipOwner",
            "FieldMappingConfigProcessType",
            "FieldOverride",
            "FieldMappingField",
            "FieldSrcTrgtRelationship",
            "FieldSetItem",
            "FieldMappingRow",
            "FieldMappingConfig",
            "FieldMapping",
            "FieldSourceTargetMap",
            "FieldSetTranslation",
            "FieldValue",
            "FieldInstance",
            "FieldMappingConfigItem",
            "FieldInstanceProperty",
            "FieldRestrictionRule",
            "ResourceTransform",
            "ResourceInitializationType",
        ],
    },
    // ==================== PHASE 2 CATEGORIES ====================

    // Einstein Copilot / Agentforce
    TypeCategory {
        name: "copilot",
        feature: "copilot",
        module_path: "metadata/copilot.rs",
        description: "Einstein Copilot, Agentforce, Assistant definitions",
        patterns: &["Assistant*"],
        explicit_types: &[
            "AssistantDefinition",
            "AssistantDefinitionStatus",
            "AssistantDefinitionProfile",
            "AssistantContextItem",
            "AssistantSkillType",
            "AssistantSkillSobjectActionType",
            "AssistantSkillSobjectAction",
            "AssistantSkillSobjectParam",
            "AssistantSkillQuickAction",
            "AssistantSkillIntent",
            "AssistantVersionAction",
        ],
    },
    // Sales Forecasting
    TypeCategory {
        name: "forecasting",
        feature: "forecasting",
        module_path: "metadata/forecasting.rs",
        description: "Sales Forecasting, Advanced Account Forecasting",
        patterns: &["Forecasting*", "AdvAcct*", "AdvAcc*", "AdvAccount*", "Adv*"],
        explicit_types: &[
            "ForecastingDateType",
            "ForecastingCategoryMapping",
            "AdvAcctFcstFormulaType",
            "AdvAcctFcstComputationMethod",
            "AdvAcctFcstMeasureType",
            "AdvAcctFcstCalcFrequency",
            "AdvAccForecastSetStatus",
            "AdvAcctFrcstDisplayGroupType",
            "AdvAcctFcstAggregationType",
            "AdvAccountForecastPeriod",
            "AdvancedObjectMapping",
            "AdvAccountForecastFormula",
            "AdvAcctFrcstDisplayGroup",
            "AdvAcctForecastDimSource",
            "AdvAcctForecastDimension",
        ],
    },
    // Next Best Action / Recommendations
    TypeCategory {
        name: "nba",
        feature: "nba",
        module_path: "metadata/nba.rs",
        description: "Next Best Action, Recommendations, Strategy Builder",
        patterns: &["Recommendation*", "Strategy*"],
        explicit_types: &[
            "RecommendationConditionValueType",
            "RecommendationConditionOperator",
            "RecommendationChannel",
            "RecommendationDefinitionDetail",
            "RecommendationDefinition",
            "RecommendationLoadCondition",
            "RecommendationAudienceDetail",
            "RecommendationStrategy",
            "RecommendationAudience",
            "RecommendationConditionValue",
            "StrategyReactionType",
            "StrategyNodeSortField",
            "StrategyActionArg",
            "StrategyNodeBase",
            "StrategyAction",
            "StrategyNodeInvocableActionArg",
        ],
    },
    // Omni-Channel
    TypeCategory {
        name: "omnichannel",
        feature: "omnichannel",
        module_path: "metadata/omnichannel.rs",
        description: "Omni-Channel routing, Presence, Skills",
        patterns: &["Presence*", "Skill*"],
        explicit_types: &[
            "PresenceConfigAssignments",
            "PresenceConfigProfileAssignments",
            "PresenceDeclineReason",
            "PresenceConfigUserAssignments",
            "PresenceUserConfig",
            "SkillType",
            "SkillUserAssignments",
            "SkillProfileAssignments",
            "Skill",
            "SkillAssignments",
            "WorkSkillRouting",
            "WorkSkillRoutingAttribute",
        ],
    },
    // Mobile Security
    TypeCategory {
        name: "mobile_security",
        feature: "mobilesecurity",
        module_path: "metadata/mobilesecurity.rs",
        description: "Mobile Security Policies, Mobile Applications",
        patterns: &["Mobile*", "MobileSecurity*"],
        explicit_types: &[
            "MobileSecurityPolicyRuleValueType",
            "MobileSecurityPolicySeverityLevel",
            "MobileSecurityPolicyType",
            "MobileSecurityCertPinType",
            "MobileSecurityMobilePlatform",
            "MobileApplicationDetail",
            "MobileSecurityPolicy",
            "MobileSecurityAssignment",
        ],
    },
    // Documents and Content
    TypeCategory {
        name: "documents",
        feature: "documents",
        module_path: "metadata/documents.rs",
        description: "Document Templates, Content Assets, OCR",
        patterns: &["Document*", "Content*", "Ocr*"],
        explicit_types: &[
            "DocumentTemplateStatus",
            "DocumentGenerationMechanism",
            "DocumentTemplateUsageType",
            "DocumentTemplateType",
            "DocumentCategoryDocumentType",
            "DocumentCategory",
            "DocumentType",
            "DocumentGenerationSetting",
            "ContentAssetAccess",
            "ContentAssetFormat",
            "ContentAssetLink",
            "ContentTypeBundleResource",
            "ContentTypeBundle",
            "ContentAssetRelationships",
            "ContentAssetVersion",
            "OcrMappingType",
            "OcrApplicationType",
            "OcrSampleDocumentPage",
            "OcrSampleDocumentPageItem",
            "OcrSampleDocument",
        ],
    },
    // Stages and Paths
    TypeCategory {
        name: "stages",
        feature: "stages",
        module_path: "metadata/stages.rs",
        description: "Sales Stages, Path Assistant, Stage Definitions",
        patterns: &["Stage*", "Path*", "Stg*"],
        explicit_types: &[
            "StageConditionOperator",
            "StageCriteriaType",
            "StageCriteriaExecType",
            "StageUserPermission",
            "StageAssignment",
            "StageTransition",
            "StageCriteria",
            "StageDefinition",
            "StageCondition",
            "StageValue",
            "PathAssistant",
            "PathAssistantStep",
            "StgAssignmentRuleCriteria",
            "StgFulfillmentStepDefGrp",
            "StgFulfillmentStepDpndDef",
        ],
    },
    // CPQ / Pricing
    TypeCategory {
        name: "cpq",
        feature: "cpq",
        module_path: "metadata/cpq.rs",
        description: "Configure-Price-Quote, Pricing Rules, Price Sheets",
        patterns: &["Price*", "Pricing*"],
        explicit_types: &[
            "PriceSheetColumnType",
            "PricingStrategy",
            "PricingElementType",
            "PriceRuleCondition",
            "PriceRuleConditionFilter",
            "PriceRuleAction",
            "PriceRule",
            "PriceSheetDefinition",
            "PricingRecipe",
            "PricingProcedureOutputMap",
        ],
    },
    // Applications
    TypeCategory {
        name: "applications",
        feature: "applications",
        module_path: "metadata/applications.rs",
        description: "Applications, App Menus, App Branding",
        patterns: &["App*", "Application*"],
        explicit_types: &[
            "ApplePushEnvironmentType",
            "ApplicationSourceType",
            "ApplicationUsageType",
            "AppDomainUsageType",
            "ApptAssistantRadiusUnit",
            "ApplicationObjectName",
            "Approver",
            "ApplicationRecordTypeConfig",
            "AppFrameworkTemplateBundle",
            "AppBrand",
            "AppComponentList",
            "ApplicationSubtypeDefinition",
            "AppMenu",
            "Application",
        ],
    },
    // Call Center / CTI
    TypeCategory {
        name: "call_center",
        feature: "callcenter",
        module_path: "metadata/callcenter.rs",
        description: "Call Center, CTI, Telephony Integration",
        patterns: &["Call*", "CallCenter*"],
        explicit_types: &[
            "CalloutStatus",
            "CallCenter",
            "CallCenterRoutingMap",
            "CallCenterSection",
            "CallCenterItem",
            "CallCoachingMediaProvider",
        ],
    },
    // Business Rules and Processes
    TypeCategory {
        name: "business_rules",
        feature: "businessrules",
        module_path: "metadata/businessrules.rs",
        description: "Business Processes, Business Knowledge Models, Rule Definitions",
        patterns: &["Business*", "Rule*"],
        explicit_types: &[
            "BusinessKnowledgeModel",
            "BusinessHoursSourceType",
            "BusinessVertical",
            "BusinessHoursEntry",
            "BusinessProcessDefinition",
            "BusinessProcessTypeDefinition",
            "BusinessProcessGroup",
            "RuleEngine",
            "RulePrincipalScopeType",
            "RuleStatus",
            "RuleResourceScopeType",
            "RuleContextPath",
            "RuleDefinitionOperator",
            "RuleDefinitionClause",
            "RuleConsumer",
            "RulePrincipalPath",
            "RuleResourcePath",
            "RuleLibraryDefinition",
            "RuleDefinition",
        ],
    },
    // Assessments / Surveys
    TypeCategory {
        name: "assessments",
        feature: "assessments",
        module_path: "metadata/assessments.rs",
        description: "Assessments, Surveys, Question Sets",
        patterns: &["Assessment*"],
        explicit_types: &[
            "AssessmentConfigurationOption",
            "AssessmentType",
            "AssessmentQuestionSet",
            "AssessmentDefinitionMetadata",
            "AssessmentQuestionVersion",
            "AssessmentConfiguration",
        ],
    },
    // Visualization / Einstein Analytics
    TypeCategory {
        name: "visualization",
        feature: "visualization",
        module_path: "metadata/visualization.rs",
        description: "Einstein Analytics Visualizations, Charts",
        patterns: &["Visualization*"],
        explicit_types: &[
            "VisualizationFieldType",
            "VisualizationFieldDisplayCategoryType",
            "VisualizationFieldFunctionType",
            "VisualizationFieldRoleType",
            "VisualizationResourceType",
            "VisualizationPlugin",
            "VisualizationType",
        ],
    },
    // Telemetry
    TypeCategory {
        name: "telemetry",
        feature: "telemetry",
        module_path: "metadata/telemetry.rs",
        description: "Telemetry Definitions, Usage Tracking",
        patterns: &["Telemetry*"],
        explicit_types: &[
            "TelemetryDefinitionUsageType",
            "TelemetryActnDefStepOpType",
            "TelemetryActnDefStepAttrType",
            "TelemetryActnDefStepAttr",
            "TelemetryActionDefStep",
            "TelemetryDefinitionVersion",
        ],
    },
    // Navigation
    TypeCategory {
        name: "navigation",
        feature: "navigation",
        module_path: "metadata/navigation.rs",
        description: "Navigation Menus, Navigation Links",
        patterns: &["Navigation*"],
        explicit_types: &["NavigationLinkSet", "NavigationSubMenu", "NavigationMenu"],
    },
    // Transaction Security
    TypeCategory {
        name: "transaction_security",
        feature: "transactionsecurity",
        module_path: "metadata/transactionsecurity.rs",
        description: "Transaction Security Policies, Event Monitoring",
        patterns: &["Transaction*"],
        explicit_types: &[
            "TransactionSecurityEventName",
            "TransactionProcessingType",
            "TransactionSecurityNotification",
            "TransactionSecurityAction",
            "TransactionSecurityPolicy",
        ],
    },
    // Invocable Actions
    TypeCategory {
        name: "invocable",
        feature: "invocable",
        module_path: "metadata/invocable.rs",
        description: "Invocable Actions, Action Extensions",
        patterns: &["Invocable*"],
        explicit_types: &[
            "InvocableActionType",
            "InvocableActionExtAttributeDataType",
            "InvocableActionExtTargetType",
            "InvocableActionExtensionTarget",
            "InvocableActionExtensionTargetAttribute",
            "InvocableActionExtension",
        ],
    },
    // Channels
    TypeCategory {
        name: "channels",
        feature: "channels",
        module_path: "metadata/channels.rs",
        description: "Channel Definitions, Channel Layouts",
        patterns: &["Channel*"],
        explicit_types: &[
            "ChannelSource",
            "Channel",
            "ChannelType",
            "ChannelMode",
            "ChannelLayout",
        ],
    },
    // Folders and Public Access
    TypeCategory {
        name: "folders",
        feature: "folders",
        module_path: "metadata/folders.rs",
        description: "Folders, Folder Sharing, Public Access",
        patterns: &["Folder*", "Public*"],
        explicit_types: &[
            "PublicKeyCertificateSetType",
            "PublicFolderAccess",
            "FolderShareAccessLevel",
            "FolderSharedToType",
            "FolderAccessTypes",
            "PublicKeyCertificateSetKey",
            "PublicKeyCertificateSet",
            "FolderShare",
            "PublicGroups",
            "PublicKeyCertificate",
        ],
    },
    // Reputation
    TypeCategory {
        name: "reputation",
        feature: "reputation",
        module_path: "metadata/reputation.rs",
        description: "Community Reputation, Points, Levels",
        patterns: &["Reputation*"],
        explicit_types: &[
            "ReputationLevels",
            "ReputationPointsRule",
            "ReputationLevel",
            "ReputationLevelDefinitions",
        ],
    },
    // Digital Experience
    TypeCategory {
        name: "digital_experience",
        feature: "digitalexperience",
        module_path: "metadata/digitalexperience.rs",
        description: "Digital Experience Configuration",
        patterns: &["Digital*"],
        explicit_types: &[
            "DigitalExperienceConfig",
            "DigitalExperienceFolderShares",
            "DigitalExperienceModule",
            "DigitalExperienceModuleCollection",
            "DigitalExperienceFolderShare",
        ],
    },
    // Briefcase (Field Service)
    TypeCategory {
        name: "briefcase",
        feature: "briefcase",
        module_path: "metadata/briefcase.rs",
        description: "Field Service Briefcase, Offline Data",
        patterns: &["Briefcase*"],
        explicit_types: &[
            "BriefcaseType",
            "BriefcaseRuleRelationshipType",
            "BriefcaseFilterOperator",
            "BriefcaseRuleFilter",
            "BriefcaseRule",
            "BriefcaseDefinition",
        ],
    },
    // Life Sciences Cloud
    TypeCategory {
        name: "life_sciences",
        feature: "lifesciences",
        module_path: "metadata/lifesciences.rs",
        description: "Life Sciences Cloud, Clinical, Healthcare",
        patterns: &["LifeSci*"],
        explicit_types: &[
            "LifeSciAssignmentLevel",
            "LifeSciCategoryType",
            "LifeSciConfigFieldDataType",
            "LifeSciConfigCategoryType",
            "LifeSciConfigFieldValue",
            "LifeSciConfigRecord",
            "LifeSciConfigCategory",
        ],
    },
    // Translations
    TypeCategory {
        name: "translations",
        feature: "translations",
        module_path: "metadata/translations.rs",
        description: "Translations, Localization",
        patterns: &["Translation*", "Locale*", "Language*"],
        explicit_types: &[
            "TranslationAspect",
            "Translations",
            "StandardFieldTranslation",
        ],
    },
    // Managed Content
    TypeCategory {
        name: "managed_content",
        feature: "managedcontent",
        module_path: "metadata/managedcontent.rs",
        description: "Managed Content, Managed Topics, CMS",
        patterns: &["Managed*"],
        explicit_types: &[
            "ManagedContentSpaceModuleStatusEnum",
            "ManagedContentNodeType",
            "ManagedTopics",
            "ManagedContentTypeBundle",
            "ManagedEventSubscription",
        ],
    },
    // Object Mappings
    TypeCategory {
        name: "object_mappings",
        feature: "objectmappings",
        module_path: "metadata/objectmappings.rs",
        description: "Object Mappings, Object Relationships",
        patterns: &["Object*"],
        explicit_types: &[
            "ObjectToLink",
            "ObjectiveParameterKey",
            "ObjectFilterOperator",
            "ObjectAccessLevel",
            "ObjectRelationshipType",
            "ObjectUsage",
            "ObjectMappingField",
            "ObjectRelationship",
            "ObjectSourceTargetMap",
            "ObjectSearchSetting",
            "ObjectNameCaseValue",
        ],
    },
    // Components
    TypeCategory {
        name: "components",
        feature: "components",
        module_path: "metadata/components.rs",
        description: "Component Instances, Component Configuration",
        patterns: &["Component*"],
        explicit_types: &[
            "ComponentInstanceType",
            "ComponentName",
            "ComponentInstance",
            "ComponentInstancePropertyListItem",
        ],
    },
    // Feed
    TypeCategory {
        name: "feed",
        feature: "feed",
        module_path: "metadata/feed.rs",
        description: "Feed Items, Feed Layouts, Chatter Feed",
        patterns: &["Feed*"],
        explicit_types: &[
            "FeedItemType",
            "FeedLayoutComponentType",
            "FeedItemDisplayFormat",
            "FeedLayoutFilterPosition",
            "FeedLayoutFilterType",
            "FeedItemVisibility",
        ],
    },
    // Code Coverage
    TypeCategory {
        name: "code_coverage",
        feature: "codecoverage",
        module_path: "metadata/codecoverage.rs",
        description: "Apex Code Coverage, Test Results",
        patterns: &["Code*"],
        explicit_types: &[
            "CodeSubType",
            "CodeFeature",
            "CodeProvisioningStatus",
            "CodeProvider",
            "CodeStatus",
            "CodeCoverageResult",
            "CodeCoverageWarning",
            "CodeLocation",
        ],
    },
    // Slack Integration
    TypeCategory {
        name: "slack",
        feature: "slack",
        module_path: "metadata/slack.rs",
        description: "Slack Integration, Slack Record Layouts",
        patterns: &["Slack*"],
        explicit_types: &["SlackRecordLayoutViewMode", "SlackRecordLayout"],
    },
    // Explainability
    TypeCategory {
        name: "explainability",
        feature: "explainability",
        module_path: "metadata/explainability.rs",
        description: "AI Explainability, Message Templates",
        patterns: &["Explainability*"],
        explicit_types: &[
            "ExplainabilityMsgTemplate",
            "ExplainabilityActionVersion",
            "ExplainabilityMsgTemplateFieldTranslation",
            "ExplainabilityMessageTemplateTokenMapping",
            "ExplainabilityActionDefinition",
        ],
    },
    // Portals
    TypeCategory {
        name: "portals",
        feature: "portals",
        module_path: "metadata/portals.rs",
        description: "Portal Configuration, Portal Roles",
        patterns: &["Portal*"],
        explicit_types: &[
            "PortalType",
            "PortalRoles",
            "PortalDelegablePermissionSet",
            "Portal",
        ],
    },
    // Picklists
    TypeCategory {
        name: "picklists",
        feature: "picklists",
        module_path: "metadata/picklists.rs",
        description: "Picklist Values, Picklist Translations",
        patterns: &["Picklist*"],
        explicit_types: &["Picklist", "PicklistValueTranslation", "PicklistValue"],
    },
    // Related Records
    TypeCategory {
        name: "related_records",
        feature: "relatedrecords",
        module_path: "metadata/relatedrecords.rs",
        description: "Related Records, Related Content",
        patterns: &["Related*"],
        explicit_types: &[
            "RelatedRecordAccessDefStatus",
            "RelatedRecordAccessDefShareTo",
            "RelatedContent",
            "RelatedRecordAccessMap",
            "RelatedRecordAssocCriteria",
            "RelatedRecordAccessFltr",
            "RelatedContentItem",
        ],
    },
    // Home Page
    TypeCategory {
        name: "home_page",
        feature: "homepage",
        module_path: "metadata/homepage.rs",
        description: "Home Page Components, Home Page Layouts",
        patterns: &["HomePage*", "Home*"],
        explicit_types: &["HomePageComponent", "HomePageLayout"],
    },
    // UI Configuration
    TypeCategory {
        name: "ui_config",
        feature: "uiconfig",
        module_path: "metadata/uiconfig.rs",
        description: "UI Configuration, UI Formulas, UI Formats",
        patterns: &["Ui*"],
        explicit_types: &[
            "UiType",
            "UiLoginFlowType",
            "UiFormatSpecificationSet",
            "UiFormatSpecification",
            "UiFormulaCriterion",
        ],
    },
    // Conditions
    TypeCategory {
        name: "conditions",
        feature: "conditions",
        module_path: "metadata/conditions.rs",
        description: "Condition Types, Condition Filters",
        patterns: &["Condition*"],
        explicit_types: &[
            "ConditionType",
            "ConditionFilterOperator",
            "ConditionLogic",
            "ConditionFilterInputValueType",
            "ConditionAggregationFunction",
        ],
    },
    // Policy Configuration
    TypeCategory {
        name: "policies",
        feature: "policies",
        module_path: "metadata/policies.rs",
        description: "Security Policies, Policy Rules, Policy Definitions",
        patterns: &["Policy*"],
        explicit_types: &[
            "PolicyAction",
            "PolicyApplicableDuration",
            "PolicyRuleDefinitionSet",
            "PolicyRuleResourceDomain",
            "PolicyRuleDefinitionCondition",
            "PolicyRuleDefinitionClauseConjunction",
        ],
    },
    // Features
    TypeCategory {
        name: "features",
        feature: "features",
        module_path: "metadata/features.rs",
        description: "Feature Parameters, Feature Configuration",
        patterns: &["Feature*"],
        explicit_types: &[
            "FeatureInputType",
            "FeatureParameterDataflowDirection",
            "FeatureParameterBoolean",
            "FeatureParameterInteger",
            "FeatureParameterDate",
        ],
    },
    // Mapping Types (general)
    TypeCategory {
        name: "mappings",
        feature: "mappings",
        module_path: "metadata/mappings.rs",
        description: "General Mapping Types, Mapping Operations",
        patterns: &["Mapping*"],
        explicit_types: &[
            "MappingAlertType",
            "MappingType",
            "MappingBehaviorType",
            "MappingOperation",
            "MappingUsageType",
        ],
    },
    // Deploy Types
    TypeCategory {
        name: "deploy",
        feature: "deploy",
        module_path: "metadata/deploy.rs",
        description: "Deploy Options, Deploy Details, Deploy Problems",
        patterns: &["Deploy*"],
        explicit_types: &["DeployProblemType", "DeployDetails", "DeployOptions"],
    },
    // Lightning General (not LWC)
    TypeCategory {
        name: "lightning",
        feature: "lightning",
        module_path: "metadata/lightning.rs",
        description: "Lightning Bolt, Lightning Framework Types",
        patterns: &["LightningBolt*", "LightningMessage*", "LightningDesign*"],
        explicit_types: &[
            "LightningBoltCategory",
            "LightningDesignSystemVersion",
            "LightningMessageField",
            "LightningBoltImages",
        ],
    },
    // Workspace
    TypeCategory {
        name: "workspace",
        feature: "workspace",
        module_path: "metadata/workspace.rs",
        description: "Workspace Mappings, Console Configuration",
        patterns: &["Workspace*", "Console*", "Tab*"],
        explicit_types: &["WorkspaceMapping", "TabLimitConfig"],
    },
    // Standard Types
    TypeCategory {
        name: "standard",
        feature: "standard",
        module_path: "metadata/standard.rs",
        description: "Standard Permission Sets, Standard Fields",
        patterns: &["Standard*"],
        explicit_types: &["StandardPermissionSet", "StandardFieldTranslation"],
    },
];

/// A type that has been categorized
#[derive(Debug, Clone)]
pub struct CategorizedType {
    /// The type name
    pub name: String,
    /// The category it belongs to (None if uncategorized)
    pub category: Option<&'static TypeCategory>,
    /// Whether this is an enum (vs struct)
    pub is_enum: bool,
}

/// Categorize a list of type names
pub fn categorize_types(type_names: &[String], are_enums: bool) -> Vec<CategorizedType> {
    type_names
        .iter()
        .map(|name| CategorizedType {
            name: name.clone(),
            category: find_category(name),
            is_enum: are_enums,
        })
        .collect()
}

/// Find which category a type belongs to
pub fn find_category(type_name: &str) -> Option<&'static TypeCategory> {
    CATEGORIES.iter().find(|cat| cat.matches(type_name))
}

/// Group types by their category
pub fn group_by_category<'a>(
    types: &'a [CategorizedType],
) -> HashMap<&'static str, Vec<&'a CategorizedType>> {
    let mut groups: HashMap<&'static str, Vec<&'a CategorizedType>> = HashMap::new();

    for ct in types {
        let key = ct.category.map(|c| c.name).unwrap_or("uncategorized");
        groups.entry(key).or_default().push(ct);
    }

    groups
}

/// Get all unique feature names
pub fn get_all_features() -> Vec<&'static str> {
    CATEGORIES.iter().map(|c| c.feature).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matches_suffix() {
        assert!(pattern_matches("*Settings", "AccountSettings"));
        assert!(pattern_matches("*Settings", "SecuritySettings"));
        assert!(!pattern_matches("*Settings", "SettingsPage"));
    }

    #[test]
    fn test_pattern_matches_prefix() {
        assert!(pattern_matches("Flow*", "FlowDefinition"));
        assert!(pattern_matches("Flow*", "FlowVariable"));
        assert!(!pattern_matches("Flow*", "DataFlow"));
    }

    #[test]
    fn test_pattern_matches_exact() {
        assert!(pattern_matches("CustomObject", "CustomObject"));
        assert!(!pattern_matches("CustomObject", "CustomObjectTranslation"));
    }

    #[test]
    fn test_category_matches() {
        let settings_cat = &CATEGORIES[0];
        assert!(settings_cat.matches("AccountSettings"));
        assert!(settings_cat.matches("SecuritySettings"));
        assert!(!settings_cat.matches("Account"));
    }

    #[test]
    fn test_find_category() {
        assert!(find_category("AccountSettings").is_some());
        assert!(find_category("FlowDefinition").is_some());
        assert!(find_category("WaveApplication").is_some());
        assert!(find_category("OmniScript").is_some());
    }

    #[test]
    fn test_categorize_types() {
        let types = vec![
            "AccountSettings".to_string(),
            "FlowVariable".to_string(),
            "UnknownType".to_string(),
        ];

        let categorized = categorize_types(&types, false);

        assert_eq!(categorized.len(), 3);
        assert!(categorized[0].category.is_some());
        assert_eq!(categorized[0].category.unwrap().name, "settings");
        assert!(categorized[1].category.is_some());
        assert_eq!(categorized[1].category.unwrap().name, "flows");
        assert!(categorized[2].category.is_none());
    }

    #[test]
    fn test_new_categories() {
        // Test analytics/wave
        assert!(find_category("WaveApplication").is_some());
        assert_eq!(find_category("WaveApplication").unwrap().name, "analytics");

        // Test omnistudio
        assert!(find_category("OmniScript").is_some());
        assert_eq!(find_category("OmniScript").unwrap().name, "omnistudio");
        assert!(find_category("FlexiPage").is_some());
        assert_eq!(find_category("FlexiPage").unwrap().name, "omnistudio");

        // Test AI/ML
        assert!(find_category("MlModelType").is_some());
        assert_eq!(find_category("MlModelType").unwrap().name, "ai_ml");
        assert!(find_category("Prompt").is_some());
        assert_eq!(find_category("Prompt").unwrap().name, "ai_ml");

        // Test decision tables
        assert!(find_category("DecisionTable").is_some());
        assert_eq!(
            find_category("DecisionTable").unwrap().name,
            "decision_tables"
        );

        // Test data cloud
        assert!(find_category("DataSource").is_some());
        assert_eq!(find_category("DataSource").unwrap().name, "data_cloud");

        // Test service cloud
        assert!(find_category("EmbeddedServiceFlowConfig").is_some());
        assert_eq!(
            find_category("EmbeddedServiceFlowConfig").unwrap().name,
            "service_cloud"
        );

        // Test messaging
        assert!(find_category("MessagingChannel").is_some());
        assert_eq!(find_category("MessagingChannel").unwrap().name, "messaging");

        // Test loyalty
        assert!(find_category("LoyaltyProgramProcess").is_some());
        assert_eq!(
            find_category("LoyaltyProgramProcess").unwrap().name,
            "loyalty"
        );

        // Test identity
        assert!(find_category("SamlSsoConfig").is_some());
        assert_eq!(find_category("SamlSsoConfig").unwrap().name, "identity");

        // Test batch
        assert!(find_category("BatchProcessJobDefinition").is_some());
        assert_eq!(
            find_category("BatchProcessJobDefinition").unwrap().name,
            "batch"
        );
    }

    #[test]
    fn test_phase2_categories() {
        // Test copilot
        assert!(find_category("AssistantDefinition").is_some());
        assert_eq!(
            find_category("AssistantDefinition").unwrap().name,
            "copilot"
        );

        // Test forecasting
        assert!(find_category("ForecastingDateType").is_some());
        assert_eq!(
            find_category("ForecastingDateType").unwrap().name,
            "forecasting"
        );
        assert!(find_category("AdvAcctFcstFormulaType").is_some());
        assert_eq!(
            find_category("AdvAcctFcstFormulaType").unwrap().name,
            "forecasting"
        );

        // Test nba
        assert!(find_category("RecommendationStrategy").is_some());
        assert_eq!(find_category("RecommendationStrategy").unwrap().name, "nba");
        assert!(find_category("StrategyAction").is_some());
        assert_eq!(find_category("StrategyAction").unwrap().name, "nba");

        // Test omnichannel
        assert!(find_category("PresenceUserConfig").is_some());
        assert_eq!(
            find_category("PresenceUserConfig").unwrap().name,
            "omnichannel"
        );
        assert!(find_category("Skill").is_some());
        assert_eq!(find_category("Skill").unwrap().name, "omnichannel");

        // Test mobile_security
        assert!(find_category("MobileSecurityPolicy").is_some());
        assert_eq!(
            find_category("MobileSecurityPolicy").unwrap().name,
            "mobile_security"
        );

        // Test documents
        assert!(find_category("DocumentTemplate").is_some());
        assert_eq!(find_category("DocumentTemplate").unwrap().name, "documents");
        assert!(find_category("ContentAsset").is_some());
        assert_eq!(find_category("ContentAsset").unwrap().name, "documents");
        assert!(find_category("OcrSampleDocument").is_some());
        assert_eq!(
            find_category("OcrSampleDocument").unwrap().name,
            "documents"
        );

        // Test stages
        assert!(find_category("StageDefinition").is_some());
        assert_eq!(find_category("StageDefinition").unwrap().name, "stages");
        assert!(find_category("PathAssistant").is_some());
        assert_eq!(find_category("PathAssistant").unwrap().name, "stages");

        // Test cpq
        assert!(find_category("PriceRule").is_some());
        assert_eq!(find_category("PriceRule").unwrap().name, "cpq");
        assert!(find_category("PricingRecipe").is_some());
        assert_eq!(find_category("PricingRecipe").unwrap().name, "cpq");

        // Test applications
        assert!(find_category("AppMenu").is_some());
        assert_eq!(find_category("AppMenu").unwrap().name, "applications");

        // Test call_center
        assert!(find_category("CallCenter").is_some());
        assert_eq!(find_category("CallCenter").unwrap().name, "call_center");

        // Test business_rules
        assert!(find_category("BusinessProcessDefinition").is_some());
        assert_eq!(
            find_category("BusinessProcessDefinition").unwrap().name,
            "business_rules"
        );
        assert!(find_category("RuleDefinition").is_some());
        assert_eq!(
            find_category("RuleDefinition").unwrap().name,
            "business_rules"
        );

        // Test assessments
        assert!(find_category("AssessmentQuestionSet").is_some());
        assert_eq!(
            find_category("AssessmentQuestionSet").unwrap().name,
            "assessments"
        );

        // Test visualization
        assert!(find_category("VisualizationPlugin").is_some());
        assert_eq!(
            find_category("VisualizationPlugin").unwrap().name,
            "visualization"
        );

        // Test telemetry
        assert!(find_category("TelemetryActionDefStep").is_some());
        assert_eq!(
            find_category("TelemetryActionDefStep").unwrap().name,
            "telemetry"
        );

        // Test navigation
        assert!(find_category("NavigationMenu").is_some());
        assert_eq!(find_category("NavigationMenu").unwrap().name, "navigation");

        // Test transaction_security
        assert!(find_category("TransactionSecurityPolicy").is_some());
        assert_eq!(
            find_category("TransactionSecurityPolicy").unwrap().name,
            "transaction_security"
        );

        // Test invocable
        assert!(find_category("InvocableActionExtension").is_some());
        assert_eq!(
            find_category("InvocableActionExtension").unwrap().name,
            "invocable"
        );

        // Test channels
        assert!(find_category("ChannelLayout").is_some());
        assert_eq!(find_category("ChannelLayout").unwrap().name, "channels");

        // Test folders
        assert!(find_category("FolderShare").is_some());
        assert_eq!(find_category("FolderShare").unwrap().name, "folders");
        assert!(find_category("PublicGroups").is_some());
        assert_eq!(find_category("PublicGroups").unwrap().name, "folders");

        // Test reputation
        assert!(find_category("ReputationLevel").is_some());
        assert_eq!(find_category("ReputationLevel").unwrap().name, "reputation");

        // Test digital_experience
        assert!(find_category("DigitalExperienceConfig").is_some());
        assert_eq!(
            find_category("DigitalExperienceConfig").unwrap().name,
            "digital_experience"
        );

        // Test briefcase
        assert!(find_category("BriefcaseDefinition").is_some());
        assert_eq!(
            find_category("BriefcaseDefinition").unwrap().name,
            "briefcase"
        );

        // Test life_sciences
        assert!(find_category("LifeSciConfigRecord").is_some());
        assert_eq!(
            find_category("LifeSciConfigRecord").unwrap().name,
            "life_sciences"
        );

        // Test translations
        assert!(find_category("Translations").is_some());
        assert_eq!(find_category("Translations").unwrap().name, "translations");

        // Test managed_content
        assert!(find_category("ManagedTopics").is_some());
        assert_eq!(
            find_category("ManagedTopics").unwrap().name,
            "managed_content"
        );

        // Test slack
        assert!(find_category("SlackRecordLayout").is_some());
        assert_eq!(find_category("SlackRecordLayout").unwrap().name, "slack");

        // Test explainability
        assert!(find_category("ExplainabilityMsgTemplate").is_some());
        assert_eq!(
            find_category("ExplainabilityMsgTemplate").unwrap().name,
            "explainability"
        );

        // Test portals
        assert!(find_category("Portal").is_some());
        assert_eq!(find_category("Portal").unwrap().name, "portals");

        // Test picklists
        assert!(find_category("PicklistValue").is_some());
        assert_eq!(find_category("PicklistValue").unwrap().name, "picklists");

        // Test related_records
        assert!(find_category("RelatedContent").is_some());
        assert_eq!(
            find_category("RelatedContent").unwrap().name,
            "related_records"
        );

        // Test home_page
        assert!(find_category("HomePageLayout").is_some());
        assert_eq!(find_category("HomePageLayout").unwrap().name, "home_page");

        // Test ui_config
        assert!(find_category("UiFormatSpecification").is_some());
        assert_eq!(
            find_category("UiFormatSpecification").unwrap().name,
            "ui_config"
        );

        // Test conditions
        assert!(find_category("ConditionLogic").is_some());
        assert_eq!(find_category("ConditionLogic").unwrap().name, "conditions");

        // Test policies
        assert!(find_category("PolicyAction").is_some());
        assert_eq!(find_category("PolicyAction").unwrap().name, "policies");

        // Test features
        assert!(find_category("FeatureParameterBoolean").is_some());
        assert_eq!(
            find_category("FeatureParameterBoolean").unwrap().name,
            "features"
        );

        // Test deploy
        assert!(find_category("DeployOptions").is_some());
        assert_eq!(find_category("DeployOptions").unwrap().name, "deploy");

        // Test lightning
        assert!(find_category("LightningBoltCategory").is_some());
        assert_eq!(
            find_category("LightningBoltCategory").unwrap().name,
            "lightning"
        );

        // Test workspace
        assert!(find_category("WorkspaceMapping").is_some());
        assert_eq!(find_category("WorkspaceMapping").unwrap().name, "workspace");
    }
}
