// Auto-generated registry mapping XML root element -> concrete parser
use crate::prelude::*;
use crate::traits::XmlError;
use crate::traits::XmlSerializable;
use serde_json::Value;

pub fn parse_by_root(root: &str, xml: &str) -> Result<Value, XmlError> {
    match root {
        "ServiceScheduleConfig" => {
            // try to parse as ServiceScheduleConfig
            let parsed: Result<ServiceScheduleConfig, XmlError> = ServiceScheduleConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VisualizationType" => {
            // try to parse as VisualizationType
            let parsed: Result<VisualizationType, XmlError> = VisualizationType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActvPlatformOAuthConnector" => {
            // try to parse as ActvPlatformOAuthConnector
            let parsed: Result<ActvPlatformOAuthConnector, XmlError> = ActvPlatformOAuthConnector::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryPrescribableField" => {
            // try to parse as DiscoveryPrescribableField
            let parsed: Result<DiscoveryPrescribableField, XmlError> = DiscoveryPrescribableField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfigSkill" => {
            // try to parse as OmniSupervisorConfigSkill
            let parsed: Result<OmniSupervisorConfigSkill, XmlError> = OmniSupervisorConfigSkill::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationServiceIntegrationSettings" => {
            // try to parse as ConversationServiceIntegrationSettings
            let parsed: Result<ConversationServiceIntegrationSettings, XmlError> = ConversationServiceIntegrationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardComponentContent" => {
            // try to parse as DashboardComponentContent
            let parsed: Result<DashboardComponentContent, XmlError> = DashboardComponentContent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OrgSettings" => {
            // try to parse as OrgSettings
            let parsed: Result<OrgSettings, XmlError> = OrgSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CodeLocation" => {
            // try to parse as CodeLocation
            let parsed: Result<CodeLocation, XmlError> = CodeLocation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FilterItem" => {
            // try to parse as FilterItem
            let parsed: Result<FilterItem, XmlError> = FilterItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GatewayProviderPaymentMethodType" => {
            // try to parse as GatewayProviderPaymentMethodType
            let parsed: Result<GatewayProviderPaymentMethodType, XmlError> = GatewayProviderPaymentMethodType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordTypeTranslation" => {
            // try to parse as RecordTypeTranslation
            let parsed: Result<RecordTypeTranslation, XmlError> = RecordTypeTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StrategyNodeBase" => {
            // try to parse as StrategyNodeBase
            let parsed: Result<StrategyNodeBase, XmlError> = StrategyNodeBase::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppSettings" => {
            // try to parse as ConnectedAppSettings
            let parsed: Result<ConnectedAppSettings, XmlError> = ConnectedAppSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileFlowAccess" => {
            // try to parse as ProfileFlowAccess
            let parsed: Result<ProfileFlowAccess, XmlError> = ProfileFlowAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationConditionValue" => {
            // try to parse as RecommendationConditionValue
            let parsed: Result<RecommendationConditionValue, XmlError> = RecommendationConditionValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExperienceResources" => {
            // try to parse as ExperienceResources
            let parsed: Result<ExperienceResources, XmlError> = ExperienceResources::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PicklistEntry" => {
            // try to parse as PicklistEntry
            let parsed: Result<PicklistEntry, XmlError> = PicklistEntry::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GlobalValueSet" => {
            // try to parse as GlobalValueSet
            let parsed: Result<GlobalValueSet, XmlError> = GlobalValueSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantSkillSobjectParam" => {
            // try to parse as AssistantSkillSobjectParam
            let parsed: Result<AssistantSkillSobjectParam, XmlError> = AssistantSkillSobjectParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchLayouts" => {
            // try to parse as SearchLayouts
            let parsed: Result<SearchLayouts, XmlError> = SearchLayouts::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationSystemMessage" => {
            // try to parse as ConversationSystemMessage
            let parsed: Result<ConversationSystemMessage, XmlError> = ConversationSystemMessage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileApplicationVisibility" => {
            // try to parse as ProfileApplicationVisibility
            let parsed: Result<ProfileApplicationVisibility, XmlError> = ProfileApplicationVisibility::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdDate" => {
            // try to parse as WaveXmdDate
            let parsed: Result<WaveXmdDate, XmlError> = WaveXmdDate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryDeployedModel" => {
            // try to parse as DiscoveryDeployedModel
            let parsed: Result<DiscoveryDeployedModel, XmlError> = DiscoveryDeployedModel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StageCriteria" => {
            // try to parse as StageCriteria
            let parsed: Result<StageCriteria, XmlError> = StageCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MfgServiceConsoleSettings" => {
            // try to parse as MfgServiceConsoleSettings
            let parsed: Result<MfgServiceConsoleSettings, XmlError> = MfgServiceConsoleSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingChannelActionParameterMapping" => {
            // try to parse as MessagingChannelActionParameterMapping
            let parsed: Result<MessagingChannelActionParameterMapping, XmlError> = MessagingChannelActionParameterMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RunTestsResult" => {
            // try to parse as RunTestsResult
            let parsed: Result<RunTestsResult, XmlError> = RunTestsResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPromptTemplateDataProviderParam" => {
            // try to parse as GenAiPromptTemplateDataProviderParam
            let parsed: Result<GenAiPromptTemplateDataProviderParam, XmlError> = GenAiPromptTemplateDataProviderParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionMatrixDefinition" => {
            // try to parse as DecisionMatrixDefinition
            let parsed: Result<DecisionMatrixDefinition, XmlError> = DecisionMatrixDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TimeSheetTemplateAssignment" => {
            // try to parse as TimeSheetTemplateAssignment
            let parsed: Result<TimeSheetTemplateAssignment, XmlError> = TimeSheetTemplateAssignment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceFormField" => {
            // try to parse as EmbeddedServiceFormField
            let parsed: Result<EmbeddedServiceFormField, XmlError> = EmbeddedServiceFormField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceProcessAttribute" => {
            // try to parse as ServiceProcessAttribute
            let parsed: Result<ServiceProcessAttribute, XmlError> = ServiceProcessAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContentTypeBundle" => {
            // try to parse as ContentTypeBundle
            let parsed: Result<ContentTypeBundle, XmlError> = ContentTypeBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PresenceConfigAssignments" => {
            // try to parse as PresenceConfigAssignments
            let parsed: Result<PresenceConfigAssignments, XmlError> = PresenceConfigAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTaskCustomContent" => {
            // try to parse as EnablementProgramTaskCustomContent
            let parsed: Result<EnablementProgramTaskCustomContent, XmlError> = EnablementProgramTaskCustomContent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataLakeAttributes" => {
            // try to parse as MktDataLakeAttributes
            let parsed: Result<MktDataLakeAttributes, XmlError> = MktDataLakeAttributes::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjIntegProviderDefMapping" => {
            // try to parse as ObjIntegProviderDefMapping
            let parsed: Result<ObjIntegProviderDefMapping, XmlError> = ObjIntegProviderDefMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalStoragePrvdConfig" => {
            // try to parse as ExternalStoragePrvdConfig
            let parsed: Result<ExternalStoragePrvdConfig, XmlError> = ExternalStoragePrvdConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileExternalDataSourceAccess" => {
            // try to parse as ProfileExternalDataSourceAccess
            let parsed: Result<ProfileExternalDataSourceAccess, XmlError> = ProfileExternalDataSourceAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssignmentRule" => {
            // try to parse as AssignmentRule
            let parsed: Result<AssignmentRule, XmlError> = AssignmentRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileApexClassAccess" => {
            // try to parse as ProfileApexClassAccess
            let parsed: Result<ProfileApexClassAccess, XmlError> = ProfileApexClassAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Letterhead" => {
            // try to parse as Letterhead
            let parsed: Result<Letterhead, XmlError> = Letterhead::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationalIntelligenceSettings" => {
            // try to parse as ConversationalIntelligenceSettings
            let parsed: Result<ConversationalIntelligenceSettings, XmlError> = ConversationalIntelligenceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppPushConfigurablePolicies" => {
            // try to parse as ExtlClntAppPushConfigurablePolicies
            let parsed: Result<ExtlClntAppPushConfigurablePolicies, XmlError> = ExtlClntAppPushConfigurablePolicies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageParameterPrimitiveDetails" => {
            // try to parse as ConversationMessageParameterPrimitiveDetails
            let parsed: Result<ConversationMessageParameterPrimitiveDetails, XmlError> = ConversationMessageParameterPrimitiveDetails::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SchemaSettings" => {
            // try to parse as SchemaSettings
            let parsed: Result<SchemaSettings, XmlError> = SchemaSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoverySettings" => {
            // try to parse as DiscoverySettings
            let parsed: Result<DiscoverySettings, XmlError> = DiscoverySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsVizWidgetDef" => {
            // try to parse as AnalyticsVizWidgetDef
            let parsed: Result<AnalyticsVizWidgetDef, XmlError> = AnalyticsVizWidgetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LanguageSettings" => {
            // try to parse as LanguageSettings
            let parsed: Result<LanguageSettings, XmlError> = LanguageSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PicklistValue" => {
            // try to parse as PicklistValue
            let parsed: Result<PicklistValue, XmlError> = PicklistValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceCloudConsoleConfig" => {
            // try to parse as ServiceCloudConsoleConfig
            let parsed: Result<ServiceCloudConsoleConfig, XmlError> = ServiceCloudConsoleConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PresenceDeclineReason" => {
            // try to parse as PresenceDeclineReason
            let parsed: Result<PresenceDeclineReason, XmlError> = PresenceDeclineReason::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuotasSettings" => {
            // try to parse as QuotasSettings
            let parsed: Result<QuotasSettings, XmlError> = QuotasSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceAppointmentSettings" => {
            // try to parse as EmbeddedServiceAppointmentSettings
            let parsed: Result<EmbeddedServiceAppointmentSettings, XmlError> = EmbeddedServiceAppointmentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventLogObjectSettings" => {
            // try to parse as EventLogObjectSettings
            let parsed: Result<EventLogObjectSettings, XmlError> = EventLogObjectSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserEngagementSettings" => {
            // try to parse as UserEngagementSettings
            let parsed: Result<UserEngagementSettings, XmlError> = UserEngagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "HistoryRetentionPolicy" => {
            // try to parse as HistoryRetentionPolicy
            let parsed: Result<HistoryRetentionPolicy, XmlError> = HistoryRetentionPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UiPreviewMessageTabDef" => {
            // try to parse as UiPreviewMessageTabDef
            let parsed: Result<UiPreviewMessageTabDef, XmlError> = UiPreviewMessageTabDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileLayoutAssignment" => {
            // try to parse as ProfileLayoutAssignment
            let parsed: Result<ProfileLayoutAssignment, XmlError> = ProfileLayoutAssignment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTaskExternalContent" => {
            // try to parse as EnablementProgramTaskExternalContent
            let parsed: Result<EnablementProgramTaskExternalContent, XmlError> = EnablementProgramTaskExternalContent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppMobileSettings" => {
            // try to parse as ExtlClntAppMobileSettings
            let parsed: Result<ExtlClntAppMobileSettings, XmlError> = ExtlClntAppMobileSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlugin" => {
            // try to parse as GenAiPlugin
            let parsed: Result<GenAiPlugin, XmlError> = GenAiPlugin::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CmsnStmtLineItemTypConfig" => {
            // try to parse as CmsnStmtLineItemTypConfig
            let parsed: Result<CmsnStmtLineItemTypConfig, XmlError> = CmsnStmtLineItemTypConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchCustomizationExplicitFilter" => {
            // try to parse as SearchCustomizationExplicitFilter
            let parsed: Result<SearchCustomizationExplicitFilter, XmlError> = SearchCustomizationExplicitFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectMappingItem" => {
            // try to parse as ObjectMappingItem
            let parsed: Result<ObjectMappingItem, XmlError> = ObjectMappingItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityTemplatePageSetting" => {
            // try to parse as CommunityTemplatePageSetting
            let parsed: Result<CommunityTemplatePageSetting, XmlError> = CommunityTemplatePageSetting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotInvocationMapping" => {
            // try to parse as BotInvocationMapping
            let parsed: Result<BotInvocationMapping, XmlError> = BotInvocationMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PageContextVariable" => {
            // try to parse as PageContextVariable
            let parsed: Result<PageContextVariable, XmlError> = PageContextVariable::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestTopLevelWithDeclMd2" => {
            // try to parse as FtestTopLevelWithDeclMd2
            let parsed: Result<FtestTopLevelWithDeclMd2, XmlError> = FtestTopLevelWithDeclMd2::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataStreamTemplate" => {
            // try to parse as DataStreamTemplate
            let parsed: Result<DataStreamTemplate, XmlError> = DataStreamTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlIntentUtterance" => {
            // try to parse as MlIntentUtterance
            let parsed: Result<MlIntentUtterance, XmlError> = MlIntentUtterance::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTextTemplateTranslation" => {
            // try to parse as FlowTextTemplateTranslation
            let parsed: Result<FlowTextTemplateTranslation, XmlError> = FlowTextTemplateTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DuplicateRuleFilter" => {
            // try to parse as DuplicateRuleFilter
            let parsed: Result<DuplicateRuleFilter, XmlError> = DuplicateRuleFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FolderShare" => {
            // try to parse as FolderShare
            let parsed: Result<FolderShare, XmlError> = FolderShare::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApplicationRecordTypeConfig" => {
            // try to parse as ApplicationRecordTypeConfig
            let parsed: Result<ApplicationRecordTypeConfig, XmlError> = ApplicationRecordTypeConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextDefinitionReference" => {
            // try to parse as ContextDefinitionReference
            let parsed: Result<ContextDefinitionReference, XmlError> = ContextDefinitionReference::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChoiceListValue" => {
            // try to parse as ChoiceListValue
            let parsed: Result<ChoiceListValue, XmlError> = ChoiceListValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextMapping" => {
            // try to parse as ContextMapping
            let parsed: Result<ContextMapping, XmlError> = ContextMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2Model" => {
            // try to parse as Territory2Model
            let parsed: Result<Territory2Model, XmlError> = Territory2Model::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GlobalPicklistValue" => {
            // try to parse as GlobalPicklistValue
            let parsed: Result<GlobalPicklistValue, XmlError> = GlobalPicklistValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RedirectWhitelistUrl" => {
            // try to parse as RedirectWhitelistUrl
            let parsed: Result<RedirectWhitelistUrl, XmlError> = RedirectWhitelistUrl::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PromptTranslation" => {
            // try to parse as PromptTranslation
            let parsed: Result<PromptTranslation, XmlError> = PromptTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextDefinitionFilter" => {
            // try to parse as ContextDefinitionFilter
            let parsed: Result<ContextDefinitionFilter, XmlError> = ContextDefinitionFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotDialogGroup" => {
            // try to parse as BotDialogGroup
            let parsed: Result<BotDialogGroup, XmlError> = BotDialogGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DatasetImportRequest" => {
            // try to parse as DatasetImportRequest
            let parsed: Result<DatasetImportRequest, XmlError> = DatasetImportRequest::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlSlotClassValue" => {
            // try to parse as MlSlotClassValue
            let parsed: Result<MlSlotClassValue, XmlError> = MlSlotClassValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DynamicFulfillmentOrchestratorSettings" => {
            // try to parse as DynamicFulfillmentOrchestratorSettings
            let parsed: Result<DynamicFulfillmentOrchestratorSettings, XmlError> = DynamicFulfillmentOrchestratorSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharingReason" => {
            // try to parse as SharingReason
            let parsed: Result<SharingReason, XmlError> = SharingReason::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ListPlacement" => {
            // try to parse as ListPlacement
            let parsed: Result<ListPlacement, XmlError> = ListPlacement::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReadResult" => {
            // try to parse as ReadResult
            let parsed: Result<ReadResult, XmlError> = ReadResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetDefinition" => {
            // try to parse as ExpressionSetDefinition
            let parsed: Result<ExpressionSetDefinition, XmlError> = ExpressionSetDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IdeasSettings" => {
            // try to parse as IdeasSettings
            let parsed: Result<IdeasSettings, XmlError> = IdeasSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedRecordAssocCriteria" => {
            // try to parse as RelatedRecordAssocCriteria
            let parsed: Result<RelatedRecordAssocCriteria, XmlError> = RelatedRecordAssocCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchOrgWideFieldConfig" => {
            // try to parse as SearchOrgWideFieldConfig
            let parsed: Result<SearchOrgWideFieldConfig, XmlError> = SearchOrgWideFieldConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SiteSettings" => {
            // try to parse as SiteSettings
            let parsed: Result<SiteSettings, XmlError> = SiteSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AgentforceForDevelopersSettings" => {
            // try to parse as AgentforceForDevelopersSettings
            let parsed: Result<AgentforceForDevelopersSettings, XmlError> = AgentforceForDevelopersSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAcctForecastAdjPeriod" => {
            // try to parse as AdvAcctForecastAdjPeriod
            let parsed: Result<AdvAcctForecastAdjPeriod, XmlError> = AdvAcctForecastAdjPeriod::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LayoutColumn" => {
            // try to parse as LayoutColumn
            let parsed: Result<LayoutColumn, XmlError> = LayoutColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomObjectTranslation" => {
            // try to parse as CustomObjectTranslation
            let parsed: Result<CustomObjectTranslation, XmlError> = CustomObjectTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RestrictionRule" => {
            // try to parse as RestrictionRule
            let parsed: Result<RestrictionRule, XmlError> = RestrictionRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcessParameter" => {
            // try to parse as LoyaltyProgramProcessParameter
            let parsed: Result<LoyaltyProgramProcessParameter, XmlError> = LoyaltyProgramProcessParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LayoutTranslation" => {
            // try to parse as LayoutTranslation
            let parsed: Result<LayoutTranslation, XmlError> = LayoutTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RetrievalSummaryDefObject" => {
            // try to parse as RetrievalSummaryDefObject
            let parsed: Result<RetrievalSummaryDefObject, XmlError> = RetrievalSummaryDefObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileCustomPermissions" => {
            // try to parse as ProfileCustomPermissions
            let parsed: Result<ProfileCustomPermissions, XmlError> = ProfileCustomPermissions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcessAction" => {
            // try to parse as LoyaltyProgramProcessAction
            let parsed: Result<LoyaltyProgramProcessAction, XmlError> = LoyaltyProgramProcessAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AgentPlatformSettings" => {
            // try to parse as AgentPlatformSettings
            let parsed: Result<AgentPlatformSettings, XmlError> = AgentPlatformSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppConfigurablePolicies" => {
            // try to parse as ExtlClntAppConfigurablePolicies
            let parsed: Result<ExtlClntAppConfigurablePolicies, XmlError> = ExtlClntAppConfigurablePolicies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordAlertCategory" => {
            // try to parse as RecordAlertCategory
            let parsed: Result<RecordAlertCategory, XmlError> = RecordAlertCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomFieldBinding" => {
            // try to parse as CustomFieldBinding
            let parsed: Result<CustomFieldBinding, XmlError> = CustomFieldBinding::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingObjectListSettings" => {
            // try to parse as ForecastingObjectListSettings
            let parsed: Result<ForecastingObjectListSettings, XmlError> = ForecastingObjectListSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LicensedCustomPermissions" => {
            // try to parse as LicensedCustomPermissions
            let parsed: Result<LicensedCustomPermissions, XmlError> = LicensedCustomPermissions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IdentityProviderSettings" => {
            // try to parse as IdentityProviderSettings
            let parsed: Result<IdentityProviderSettings, XmlError> = IdentityProviderSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AgentConfigProfileAssignments" => {
            // try to parse as AgentConfigProfileAssignments
            let parsed: Result<AgentConfigProfileAssignments, XmlError> = AgentConfigProfileAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataCategoryGroup" => {
            // try to parse as DataCategoryGroup
            let parsed: Result<DataCategoryGroup, XmlError> = DataCategoryGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobCustomNodeParameter" => {
            // try to parse as BatchCalcJobCustomNodeParameter
            let parsed: Result<BatchCalcJobCustomNodeParameter, XmlError> = BatchCalcJobCustomNodeParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomDataTypeComponent" => {
            // try to parse as CustomDataTypeComponent
            let parsed: Result<CustomDataTypeComponent, XmlError> = CustomDataTypeComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LeadConfigSettings" => {
            // try to parse as LeadConfigSettings
            let parsed: Result<LeadConfigSettings, XmlError> = LeadConfigSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveChatButtonSkills" => {
            // try to parse as LiveChatButtonSkills
            let parsed: Result<LiveChatButtonSkills, XmlError> = LiveChatButtonSkills::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DisclosureDefinitionVersion" => {
            // try to parse as DisclosureDefinitionVersion
            let parsed: Result<DisclosureDefinitionVersion, XmlError> = DisclosureDefinitionVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkSkillRoutingAttribute" => {
            // try to parse as WorkSkillRoutingAttribute
            let parsed: Result<WorkSkillRoutingAttribute, XmlError> = WorkSkillRoutingAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActivationPlatformActvAttr" => {
            // try to parse as ActivationPlatformActvAttr
            let parsed: Result<ActivationPlatformActvAttr, XmlError> = ActivationPlatformActvAttr::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AddressSettings" => {
            // try to parse as AddressSettings
            let parsed: Result<AddressSettings, XmlError> = AddressSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowInputValidationRule" => {
            // try to parse as FlowInputValidationRule
            let parsed: Result<FlowInputValidationRule, XmlError> = FlowInputValidationRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobOrderByField" => {
            // try to parse as BatchCalcJobOrderByField
            let parsed: Result<BatchCalcJobOrderByField, XmlError> = BatchCalcJobOrderByField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MeetingsSettings" => {
            // try to parse as MeetingsSettings
            let parsed: Result<MeetingsSettings, XmlError> = MeetingsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NtfcnChannelRec" => {
            // try to parse as NtfcnChannelRec
            let parsed: Result<NtfcnChannelRec, XmlError> = NtfcnChannelRec::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ClaimFinancialSettings" => {
            // try to parse as ClaimFinancialSettings
            let parsed: Result<ClaimFinancialSettings, XmlError> = ClaimFinancialSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceCustomLabel" => {
            // try to parse as EmbeddedServiceCustomLabel
            let parsed: Result<EmbeddedServiceCustomLabel, XmlError> = EmbeddedServiceCustomLabel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EinsteinDealInsightsSettings" => {
            // try to parse as EinsteinDealInsightsSettings
            let parsed: Result<EinsteinDealInsightsSettings, XmlError> = EinsteinDealInsightsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesAutomotiveSettings" => {
            // try to parse as IndustriesAutomotiveSettings
            let parsed: Result<IndustriesAutomotiveSettings, XmlError> = IndustriesAutomotiveSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NextAutomatedApprover" => {
            // try to parse as NextAutomatedApprover
            let parsed: Result<NextAutomatedApprover, XmlError> = NextAutomatedApprover::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharedTo" => {
            // try to parse as SharedTo
            let parsed: Result<SharedTo, XmlError> = SharedTo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomDataType" => {
            // try to parse as CustomDataType
            let parsed: Result<CustomDataType, XmlError> = CustomDataType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PaymentsSharingSettings" => {
            // try to parse as PaymentsSharingSettings
            let parsed: Result<PaymentsSharingSettings, XmlError> = PaymentsSharingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MarketingAppExtAction" => {
            // try to parse as MarketingAppExtAction
            let parsed: Result<MarketingAppExtAction, XmlError> = MarketingAppExtAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SocialCustomerServiceSettings" => {
            // try to parse as SocialCustomerServiceSettings
            let parsed: Result<SocialCustomerServiceSettings, XmlError> = SocialCustomerServiceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogItemDefDataCategorySelection" => {
            // try to parse as SvcCatalogItemDefDataCategorySelection
            let parsed: Result<SvcCatalogItemDefDataCategorySelection, XmlError> = SvcCatalogItemDefDataCategorySelection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldMappingConfig" => {
            // try to parse as FieldMappingConfig
            let parsed: Result<FieldMappingConfig, XmlError> = FieldMappingConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiEvalCopilotTestCaseConv" => {
            // try to parse as AiEvalCopilotTestCaseConv
            let parsed: Result<AiEvalCopilotTestCaseConv, XmlError> = AiEvalCopilotTestCaseConv::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldMappingConfigItem" => {
            // try to parse as FieldMappingConfigItem
            let parsed: Result<FieldMappingConfigItem, XmlError> = FieldMappingConfigItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NotificationTypeConfig" => {
            // try to parse as NotificationTypeConfig
            let parsed: Result<NotificationTypeConfig, XmlError> = NotificationTypeConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectMapping" => {
            // try to parse as ObjectMapping
            let parsed: Result<ObjectMapping, XmlError> = ObjectMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PublicGroups" => {
            // try to parse as PublicGroups
            let parsed: Result<PublicGroups, XmlError> = PublicGroups::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIApplication" => {
            // try to parse as AIApplication
            let parsed: Result<AIApplication, XmlError> = AIApplication::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConvIntelligenceSignalRule" => {
            // try to parse as ConvIntelligenceSignalRule
            let parsed: Result<ConvIntelligenceSignalRule, XmlError> = ConvIntelligenceSignalRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PrmCoreSettings" => {
            // try to parse as PrmCoreSettings
            let parsed: Result<PrmCoreSettings, XmlError> = PrmCoreSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldSetTranslation" => {
            // try to parse as FieldSetTranslation
            let parsed: Result<FieldSetTranslation, XmlError> = FieldSetTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ManagedContentType" => {
            // try to parse as ManagedContentType
            let parsed: Result<ManagedContentType, XmlError> = ManagedContentType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InstalledPackage" => {
            // try to parse as InstalledPackage
            let parsed: Result<InstalledPackage, XmlError> = InstalledPackage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomFieldDisplay" => {
            // try to parse as CustomFieldDisplay
            let parsed: Result<CustomFieldDisplay, XmlError> = CustomFieldDisplay::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SlackFeatureSettings" => {
            // try to parse as SlackFeatureSettings
            let parsed: Result<SlackFeatureSettings, XmlError> = SlackFeatureSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppMenu" => {
            // try to parse as AppMenu
            let parsed: Result<AppMenu, XmlError> = AppMenu::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceLayout" => {
            // try to parse as EmbeddedServiceLayout
            let parsed: Result<EmbeddedServiceLayout, XmlError> = EmbeddedServiceLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RetrieveMessage" => {
            // try to parse as RetrieveMessage
            let parsed: Result<RetrieveMessage, XmlError> = RetrieveMessage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TransactionSecurityPolicy" => {
            // try to parse as TransactionSecurityPolicy
            let parsed: Result<TransactionSecurityPolicy, XmlError> = TransactionSecurityPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RuleDefinition" => {
            // try to parse as RuleDefinition
            let parsed: Result<RuleDefinition, XmlError> = RuleDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NtfcnDefinition" => {
            // try to parse as NtfcnDefinition
            let parsed: Result<NtfcnDefinition, XmlError> = NtfcnDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AudienceCriteriaValue" => {
            // try to parse as AudienceCriteriaValue
            let parsed: Result<AudienceCriteriaValue, XmlError> = AudienceCriteriaValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChatterAnswersReputationLevel" => {
            // try to parse as ChatterAnswersReputationLevel
            let parsed: Result<ChatterAnswersReputationLevel, XmlError> = ChatterAnswersReputationLevel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NavigationSubMenu" => {
            // try to parse as NavigationSubMenu
            let parsed: Result<NavigationSubMenu, XmlError> = NavigationSubMenu::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppApplePushConfig" => {
            // try to parse as ExtlClntAppApplePushConfig
            let parsed: Result<ExtlClntAppApplePushConfig, XmlError> = ExtlClntAppApplePushConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TelemetryActionDefStep" => {
            // try to parse as TelemetryActionDefStep
            let parsed: Result<TelemetryActionDefStep, XmlError> = TelemetryActionDefStep::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotBlockTranslation" => {
            // try to parse as BotBlockTranslation
            let parsed: Result<BotBlockTranslation, XmlError> = BotBlockTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestSubDetailWithDeclMd" => {
            // try to parse as FtestSubDetailWithDeclMd
            let parsed: Result<FtestSubDetailWithDeclMd, XmlError> = FtestSubDetailWithDeclMd::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NavigationMenu" => {
            // try to parse as NavigationMenu
            let parsed: Result<NavigationMenu, XmlError> = NavigationMenu::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsWorkspaceAsset" => {
            // try to parse as AnalyticsWorkspaceAsset
            let parsed: Result<AnalyticsWorkspaceAsset, XmlError> = AnalyticsWorkspaceAsset::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AuraDefinitionBundle" => {
            // try to parse as AuraDefinitionBundle
            let parsed: Result<AuraDefinitionBundle, XmlError> = AuraDefinitionBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppNotificationSettings" => {
            // try to parse as ExtlClntAppNotificationSettings
            let parsed: Result<ExtlClntAppNotificationSettings, XmlError> = ExtlClntAppNotificationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PipelineInspMetricConfig" => {
            // try to parse as PipelineInspMetricConfig
            let parsed: Result<PipelineInspMetricConfig, XmlError> = PipelineInspMetricConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PriceRuleActionItem" => {
            // try to parse as PriceRuleActionItem
            let parsed: Result<PriceRuleActionItem, XmlError> = PriceRuleActionItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LocationUse" => {
            // try to parse as LocationUse
            let parsed: Result<LocationUse, XmlError> = LocationUse::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FilesConnectSettings" => {
            // try to parse as FilesConnectSettings
            let parsed: Result<FilesConnectSettings, XmlError> = FilesConnectSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdRecordDisplayLookup" => {
            // try to parse as WaveXmdRecordDisplayLookup
            let parsed: Result<WaveXmdRecordDisplayLookup, XmlError> = WaveXmdRecordDisplayLookup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RuleLibraryDefinition" => {
            // try to parse as RuleLibraryDefinition
            let parsed: Result<RuleLibraryDefinition, XmlError> = RuleLibraryDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CtxAttrHydrationCtx" => {
            // try to parse as CtxAttrHydrationCtx
            let parsed: Result<CtxAttrHydrationCtx, XmlError> = CtxAttrHydrationCtx::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppNotificationType" => {
            // try to parse as ExtlClntAppNotificationType
            let parsed: Result<ExtlClntAppNotificationType, XmlError> = ExtlClntAppNotificationType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionPlanTemplate" => {
            // try to parse as ActionPlanTemplate
            let parsed: Result<ActionPlanTemplate, XmlError> = ActionPlanTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetCustomSettingAccess" => {
            // try to parse as PermissionSetCustomSettingAccess
            let parsed: Result<PermissionSetCustomSettingAccess, XmlError> = PermissionSetCustomSettingAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageEventSourceProperty" => {
            // try to parse as FlexiPageEventSourceProperty
            let parsed: Result<FlexiPageEventSourceProperty, XmlError> = FlexiPageEventSourceProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PackageVersion" => {
            // try to parse as PackageVersion
            let parsed: Result<PackageVersion, XmlError> = PackageVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobWritebackMapping" => {
            // try to parse as BatchCalcJobWritebackMapping
            let parsed: Result<BatchCalcJobWritebackMapping, XmlError> = BatchCalcJobWritebackMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextAttribute" => {
            // try to parse as ContextAttribute
            let parsed: Result<ContextAttribute, XmlError> = ContextAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PolicyRuleResourceDomain" => {
            // try to parse as PolicyRuleResourceDomain
            let parsed: Result<PolicyRuleResourceDomain, XmlError> = PolicyRuleResourceDomain::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementMeasureSourceObjectDefinition" => {
            // try to parse as EnablementMeasureSourceObjectDefinition
            let parsed: Result<EnablementMeasureSourceObjectDefinition, XmlError> = EnablementMeasureSourceObjectDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PricingRecipe" => {
            // try to parse as PricingRecipe
            let parsed: Result<PricingRecipe, XmlError> = PricingRecipe::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Country" => {
            // try to parse as Country
            let parsed: Result<Country, XmlError> = Country::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UnnamedChildFTestMd1" => {
            // try to parse as UnnamedChildFTestMd1
            let parsed: Result<UnnamedChildFTestMd1, XmlError> = UnnamedChildFTestMd1::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationSystemDialog" => {
            // try to parse as ConversationSystemDialog
            let parsed: Result<ConversationSystemDialog, XmlError> = ConversationSystemDialog::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StrategyActionArg" => {
            // try to parse as StrategyActionArg
            let parsed: Result<StrategyActionArg, XmlError> = StrategyActionArg::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VoiceSettings" => {
            // try to parse as VoiceSettings
            let parsed: Result<VoiceSettings, XmlError> = VoiceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Queue" => {
            // try to parse as Queue
            let parsed: Result<Queue, XmlError> = Queue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoginFlow" => {
            // try to parse as LoginFlow
            let parsed: Result<LoginFlow, XmlError> = LoginFlow::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OutboundNetworkConnProperty" => {
            // try to parse as OutboundNetworkConnProperty
            let parsed: Result<OutboundNetworkConnProperty, XmlError> = OutboundNetworkConnProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExperienceBundle" => {
            // try to parse as ExperienceBundle
            let parsed: Result<ExperienceBundle, XmlError> = ExperienceBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MediaAgentSettings" => {
            // try to parse as MediaAgentSettings
            let parsed: Result<MediaAgentSettings, XmlError> = MediaAgentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AudienceCriterion" => {
            // try to parse as AudienceCriterion
            let parsed: Result<AudienceCriterion, XmlError> = AudienceCriterion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CatalogedApi" => {
            // try to parse as CatalogedApi
            let parsed: Result<CatalogedApi, XmlError> = CatalogedApi::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryFilter" => {
            // try to parse as DiscoveryFilter
            let parsed: Result<DiscoveryFilter, XmlError> = DiscoveryFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AgentConfigUserAssignments" => {
            // try to parse as AgentConfigUserAssignments
            let parsed: Result<AgentConfigUserAssignments, XmlError> = AgentConfigUserAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingGroup" => {
            // try to parse as ForecastingGroup
            let parsed: Result<ForecastingGroup, XmlError> = ForecastingGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsVizField" => {
            // try to parse as AnalyticsVizField
            let parsed: Result<AnalyticsVizField, XmlError> = AnalyticsVizField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StageValue" => {
            // try to parse as StageValue
            let parsed: Result<StageValue, XmlError> = StageValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExperiencePropertyTypeBundleResource" => {
            // try to parse as ExperiencePropertyTypeBundleResource
            let parsed: Result<ExperiencePropertyTypeBundleResource, XmlError> = ExperiencePropertyTypeBundleResource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtDataTranObjectTemplate" => {
            // try to parse as ExtDataTranObjectTemplate
            let parsed: Result<ExtDataTranObjectTemplate, XmlError> = ExtDataTranObjectTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveAnalyticAssetCollection" => {
            // try to parse as WaveAnalyticAssetCollection
            let parsed: Result<WaveAnalyticAssetCollection, XmlError> = WaveAnalyticAssetCollection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageOptionsParameter" => {
            // try to parse as ConversationMessageOptionsParameter
            let parsed: Result<ConversationMessageOptionsParameter, XmlError> = ConversationMessageOptionsParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdFormattingPredicate" => {
            // try to parse as WaveXmdFormattingPredicate
            let parsed: Result<WaveXmdFormattingPredicate, XmlError> = WaveXmdFormattingPredicate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomValue" => {
            // try to parse as CustomValue
            let parsed: Result<CustomValue, XmlError> = CustomValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PortalsSettings" => {
            // try to parse as PortalsSettings
            let parsed: Result<PortalsSettings, XmlError> = PortalsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PersonalizationTargetInfo" => {
            // try to parse as PersonalizationTargetInfo
            let parsed: Result<PersonalizationTargetInfo, XmlError> = PersonalizationTargetInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EscalationRules" => {
            // try to parse as EscalationRules
            let parsed: Result<EscalationRules, XmlError> = EscalationRules::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ClaimMgmtFoundationEnabledSettings" => {
            // try to parse as ClaimMgmtFoundationEnabledSettings
            let parsed: Result<ClaimMgmtFoundationEnabledSettings, XmlError> = ClaimMgmtFoundationEnabledSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAcctForecastPeriodGroup" => {
            // try to parse as AdvAcctForecastPeriodGroup
            let parsed: Result<AdvAcctForecastPeriodGroup, XmlError> = AdvAcctForecastPeriodGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataKitObjectTemplate" => {
            // try to parse as DataKitObjectTemplate
            let parsed: Result<DataKitObjectTemplate, XmlError> = DataKitObjectTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomNotificationActionDefinition" => {
            // try to parse as CustomNotificationActionDefinition
            let parsed: Result<CustomNotificationActionDefinition, XmlError> = CustomNotificationActionDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LicensingSettings" => {
            // try to parse as LicensingSettings
            let parsed: Result<LicensingSettings, XmlError> = LicensingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlannerRuleExprAsgn" => {
            // try to parse as GenAiPlannerRuleExprAsgn
            let parsed: Result<GenAiPlannerRuleExprAsgn, XmlError> = GenAiPlannerRuleExprAsgn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAcctForecastMeasureDef" => {
            // try to parse as AdvAcctForecastMeasureDef
            let parsed: Result<AdvAcctForecastMeasureDef, XmlError> = AdvAcctForecastMeasureDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StrategyNodeSortField" => {
            // try to parse as StrategyNodeSortField
            let parsed: Result<StrategyNodeSortField, XmlError> = StrategyNodeSortField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EclairMap" => {
            // try to parse as EclairMap
            let parsed: Result<EclairMap, XmlError> = EclairMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPromptTemplateGenerationConfig" => {
            // try to parse as GenAiPromptTemplateGenerationConfig
            let parsed: Result<GenAiPromptTemplateGenerationConfig, XmlError> = GenAiPromptTemplateGenerationConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardComponentGroupingSortProperties" => {
            // try to parse as DashboardComponentGroupingSortProperties
            let parsed: Result<DashboardComponentGroupingSortProperties, XmlError> = DashboardComponentGroupingSortProperties::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppSessionPolicy" => {
            // try to parse as ConnectedAppSessionPolicy
            let parsed: Result<ConnectedAppSessionPolicy, XmlError> = ConnectedAppSessionPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageHandler" => {
            // try to parse as ConversationMessageHandler
            let parsed: Result<ConversationMessageHandler, XmlError> = ConversationMessageHandler::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantSkillQuickActionParam" => {
            // try to parse as AssistantSkillQuickActionParam
            let parsed: Result<AssistantSkillQuickActionParam, XmlError> = AssistantSkillQuickActionParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OpportunityListFieldsUnselectedSettings" => {
            // try to parse as OpportunityListFieldsUnselectedSettings
            let parsed: Result<OpportunityListFieldsUnselectedSettings, XmlError> = OpportunityListFieldsUnselectedSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlDomain" => {
            // try to parse as MlDomain
            let parsed: Result<MlDomain, XmlError> = MlDomain::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FTestSettings" => {
            // try to parse as FTestSettings
            let parsed: Result<FTestSettings, XmlError> = FTestSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeedLayout" => {
            // try to parse as FeedLayout
            let parsed: Result<FeedLayout, XmlError> = FeedLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceChannelFieldPriority" => {
            // try to parse as ServiceChannelFieldPriority
            let parsed: Result<ServiceChannelFieldPriority, XmlError> = ServiceChannelFieldPriority::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TransactionSecurityAction" => {
            // try to parse as TransactionSecurityAction
            let parsed: Result<TransactionSecurityAction, XmlError> = TransactionSecurityAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataPlatform" => {
            // try to parse as DataPlatform
            let parsed: Result<DataPlatform, XmlError> = DataPlatform::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ValidationRule" => {
            // try to parse as ValidationRule
            let parsed: Result<ValidationRule, XmlError> = ValidationRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantDefinitionProfile" => {
            // try to parse as AssistantDefinitionProfile
            let parsed: Result<AssistantDefinitionProfile, XmlError> = AssistantDefinitionProfile::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OrderSettings" => {
            // try to parse as OrderSettings
            let parsed: Result<OrderSettings, XmlError> = OrderSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesFieldServiceSettings" => {
            // try to parse as IndustriesFieldServiceSettings
            let parsed: Result<IndustriesFieldServiceSettings, XmlError> = IndustriesFieldServiceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationContextVariableMapping" => {
            // try to parse as ConversationContextVariableMapping
            let parsed: Result<ConversationContextVariableMapping, XmlError> = ConversationContextVariableMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchDataSource" => {
            // try to parse as BatchDataSource
            let parsed: Result<BatchDataSource, XmlError> = BatchDataSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApiNamedQueryParameter" => {
            // try to parse as ApiNamedQueryParameter
            let parsed: Result<ApiNamedQueryParameter, XmlError> = ApiNamedQueryParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingTypeSource" => {
            // try to parse as ForecastingTypeSource
            let parsed: Result<ForecastingTypeSource, XmlError> = ForecastingTypeSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ValueSettings" => {
            // try to parse as ValueSettings
            let parsed: Result<ValueSettings, XmlError> = ValueSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetLicenseDefinition" => {
            // try to parse as PermissionSetLicenseDefinition
            let parsed: Result<PermissionSetLicenseDefinition, XmlError> = PermissionSetLicenseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIFilter" => {
            // try to parse as AIFilter
            let parsed: Result<AIFilter, XmlError> = AIFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RealTimeEvent" => {
            // try to parse as RealTimeEvent
            let parsed: Result<RealTimeEvent, XmlError> = RealTimeEvent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DocumentCategory" => {
            // try to parse as DocumentCategory
            let parsed: Result<DocumentCategory, XmlError> = DocumentCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportAggregateFilter" => {
            // try to parse as ReportAggregateFilter
            let parsed: Result<ReportAggregateFilter, XmlError> = ReportAggregateFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeLanguage" => {
            // try to parse as KnowledgeLanguage
            let parsed: Result<KnowledgeLanguage, XmlError> = KnowledgeLanguage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LogInfo" => {
            // try to parse as LogInfo
            let parsed: Result<LogInfo, XmlError> = LogInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LifeSciConfigCategory" => {
            // try to parse as LifeSciConfigCategory
            let parsed: Result<LifeSciConfigCategory, XmlError> = LifeSciConfigCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IncludeEstTaxInQuoteCPQSettings" => {
            // try to parse as IncludeEstTaxInQuoteCPQSettings
            let parsed: Result<IncludeEstTaxInQuoteCPQSettings, XmlError> = IncludeEstTaxInQuoteCPQSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssessmentDefinitionMetadata" => {
            // try to parse as AssessmentDefinitionMetadata
            let parsed: Result<AssessmentDefinitionMetadata, XmlError> = AssessmentDefinitionMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BrandingSet" => {
            // try to parse as BrandingSet
            let parsed: Result<BrandingSet, XmlError> = BrandingSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductSpecificationRecType" => {
            // try to parse as ProductSpecificationRecType
            let parsed: Result<ProductSpecificationRecType, XmlError> = ProductSpecificationRecType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionTableSourceCriteria" => {
            // try to parse as DecisionTableSourceCriteria
            let parsed: Result<DecisionTableSourceCriteria, XmlError> = DecisionTableSourceCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApexEmailNotification" => {
            // try to parse as ApexEmailNotification
            let parsed: Result<ApexEmailNotification, XmlError> = ApexEmailNotification::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdFormattingBin" => {
            // try to parse as WaveXmdFormattingBin
            let parsed: Result<WaveXmdFormattingBin, XmlError> = WaveXmdFormattingBin::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SaveResult" => {
            // try to parse as SaveResult
            let parsed: Result<SaveResult, XmlError> = SaveResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MLRecommendationDefinition" => {
            // try to parse as MLRecommendationDefinition
            let parsed: Result<MLRecommendationDefinition, XmlError> = MLRecommendationDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OauthTokenExchHandlerApp" => {
            // try to parse as OauthTokenExchHandlerApp
            let parsed: Result<OauthTokenExchHandlerApp, XmlError> = OauthTokenExchHandlerApp::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DeployMessage" => {
            // try to parse as DeployMessage
            let parsed: Result<DeployMessage, XmlError> = DeployMessage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataModelAttributes" => {
            // try to parse as MktDataModelAttributes
            let parsed: Result<MktDataModelAttributes, XmlError> = MktDataModelAttributes::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingFilter" => {
            // try to parse as ForecastingFilter
            let parsed: Result<ForecastingFilter, XmlError> = ForecastingFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalAction" => {
            // try to parse as ApprovalAction
            let parsed: Result<ApprovalAction, XmlError> = ApprovalAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkflowActionReference" => {
            // try to parse as WorkflowActionReference
            let parsed: Result<WorkflowActionReference, XmlError> = WorkflowActionReference::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IPAddressRange" => {
            // try to parse as IPAddressRange
            let parsed: Result<IPAddressRange, XmlError> = IPAddressRange::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlModelInput" => {
            // try to parse as MlModelInput
            let parsed: Result<MlModelInput, XmlError> = MlModelInput::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MapExpression" => {
            // try to parse as MapExpression
            let parsed: Result<MapExpression, XmlError> = MapExpression::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionLayoutColumn" => {
            // try to parse as QuickActionLayoutColumn
            let parsed: Result<QuickActionLayoutColumn, XmlError> = QuickActionLayoutColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobFilterCriteria" => {
            // try to parse as BatchCalcJobFilterCriteria
            let parsed: Result<BatchCalcJobFilterCriteria, XmlError> = BatchCalcJobFilterCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SingleSignOnSettings" => {
            // try to parse as SingleSignOnSettings
            let parsed: Result<SingleSignOnSettings, XmlError> = SingleSignOnSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AddOnDefinition" => {
            // try to parse as AddOnDefinition
            let parsed: Result<AddOnDefinition, XmlError> = AddOnDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionLinkTemplate" => {
            // try to parse as ActionLinkTemplate
            let parsed: Result<ActionLinkTemplate, XmlError> = ActionLinkTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowCustomProperty" => {
            // try to parse as FlowCustomProperty
            let parsed: Result<FlowCustomProperty, XmlError> = FlowCustomProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldSrcTrgtRelationship" => {
            // try to parse as FieldSrcTrgtRelationship
            let parsed: Result<FieldSrcTrgtRelationship, XmlError> = FieldSrcTrgtRelationship::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionOverride" => {
            // try to parse as ActionOverride
            let parsed: Result<ActionOverride, XmlError> = ActionOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlannerRuleExprDef" => {
            // try to parse as GenAiPlannerRuleExprDef
            let parsed: Result<GenAiPlannerRuleExprDef, XmlError> = GenAiPlannerRuleExprDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EinsteinAssistantSettings" => {
            // try to parse as EinsteinAssistantSettings
            let parsed: Result<EinsteinAssistantSettings, XmlError> = EinsteinAssistantSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DevHubSettings" => {
            // try to parse as DevHubSettings
            let parsed: Result<DevHubSettings, XmlError> = DevHubSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductAttrDisplayConfig" => {
            // try to parse as ProductAttrDisplayConfig
            let parsed: Result<ProductAttrDisplayConfig, XmlError> = ProductAttrDisplayConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileAgentAccess" => {
            // try to parse as ProfileAgentAccess
            let parsed: Result<ProfileAgentAccess, XmlError> = ProfileAgentAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PicklistSettings" => {
            // try to parse as PicklistSettings
            let parsed: Result<PicklistSettings, XmlError> = PicklistSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationStrategy" => {
            // try to parse as RecommendationStrategy
            let parsed: Result<RecommendationStrategy, XmlError> = RecommendationStrategy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PromptVersionTranslation" => {
            // try to parse as PromptVersionTranslation
            let parsed: Result<PromptVersionTranslation, XmlError> = PromptVersionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BenefitActionParameterValue" => {
            // try to parse as BenefitActionParameterValue
            let parsed: Result<BenefitActionParameterValue, XmlError> = BenefitActionParameterValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UIObjectRelationFieldConfig" => {
            // try to parse as UIObjectRelationFieldConfig
            let parsed: Result<UIObjectRelationFieldConfig, XmlError> = UIObjectRelationFieldConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataPlatformDataSet" => {
            // try to parse as DataPlatformDataSet
            let parsed: Result<DataPlatformDataSet, XmlError> = DataPlatformDataSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExplainabilityMsgTemplate" => {
            // try to parse as ExplainabilityMsgTemplate
            let parsed: Result<ExplainabilityMsgTemplate, XmlError> = ExplainabilityMsgTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalSubmitter" => {
            // try to parse as ApprovalSubmitter
            let parsed: Result<ApprovalSubmitter, XmlError> = ApprovalSubmitter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceMenuItem" => {
            // try to parse as EmbeddedServiceMenuItem
            let parsed: Result<EmbeddedServiceMenuItem, XmlError> = EmbeddedServiceMenuItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardComponentColumn" => {
            // try to parse as DashboardComponentColumn
            let parsed: Result<DashboardComponentColumn, XmlError> = DashboardComponentColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CallCenterSection" => {
            // try to parse as CallCenterSection
            let parsed: Result<CallCenterSection, XmlError> = CallCenterSection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DeployResult" => {
            // try to parse as DeployResult
            let parsed: Result<DeployResult, XmlError> = DeployResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalAIModel" => {
            // try to parse as ExternalAIModel
            let parsed: Result<ExternalAIModel, XmlError> = ExternalAIModel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommandActionResponse" => {
            // try to parse as CommandActionResponse
            let parsed: Result<CommandActionResponse, XmlError> = CommandActionResponse::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceProcessDependency" => {
            // try to parse as ServiceProcessDependency
            let parsed: Result<ServiceProcessDependency, XmlError> = ServiceProcessDependency::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TabLimitConfig" => {
            // try to parse as TabLimitConfig
            let parsed: Result<TabLimitConfig, XmlError> = TabLimitConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountSharingRuleSettings" => {
            // try to parse as AccountSharingRuleSettings
            let parsed: Result<AccountSharingRuleSettings, XmlError> = AccountSharingRuleSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BldgEnrgyIntensityCnfg" => {
            // try to parse as BldgEnrgyIntensityCnfg
            let parsed: Result<BldgEnrgyIntensityCnfg, XmlError> = BldgEnrgyIntensityCnfg::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataSrcDataModelFieldMap" => {
            // try to parse as DataSrcDataModelFieldMap
            let parsed: Result<DataSrcDataModelFieldMap, XmlError> = DataSrcDataModelFieldMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingSourceDefinition" => {
            // try to parse as ForecastingSourceDefinition
            let parsed: Result<ForecastingSourceDefinition, XmlError> = ForecastingSourceDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LetterheadHeaderFooter" => {
            // try to parse as LetterheadHeaderFooter
            let parsed: Result<LetterheadHeaderFooter, XmlError> = LetterheadHeaderFooter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationRecordLookupCondition" => {
            // try to parse as ConversationRecordLookupCondition
            let parsed: Result<ConversationRecordLookupCondition, XmlError> = ConversationRecordLookupCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantVersionAction" => {
            // try to parse as AssistantVersionAction
            let parsed: Result<AssistantVersionAction, XmlError> = AssistantVersionAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextMappingIntent" => {
            // try to parse as ContextMappingIntent
            let parsed: Result<ContextMappingIntent, XmlError> = ContextMappingIntent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsSettings" => {
            // try to parse as AnalyticsSettings
            let parsed: Result<AnalyticsSettings, XmlError> = AnalyticsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionTblDatasetParameter" => {
            // try to parse as DecisionTblDatasetParameter
            let parsed: Result<DecisionTblDatasetParameter, XmlError> = DecisionTblDatasetParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EntitlementProcessMilestoneTimeTrigger" => {
            // try to parse as EntitlementProcessMilestoneTimeTrigger
            let parsed: Result<EntitlementProcessMilestoneTimeTrigger, XmlError> = EntitlementProcessMilestoneTimeTrigger::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordPageSettings" => {
            // try to parse as RecordPageSettings
            let parsed: Result<RecordPageSettings, XmlError> = RecordPageSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceItsmIntelligenceUddSettings" => {
            // try to parse as ServiceItsmIntelligenceUddSettings
            let parsed: Result<ServiceItsmIntelligenceUddSettings, XmlError> = ServiceItsmIntelligenceUddSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationAudienceDetail" => {
            // try to parse as RecommendationAudienceDetail
            let parsed: Result<RecommendationAudienceDetail, XmlError> = RecommendationAudienceDetail::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TimelineObjectDefinition" => {
            // try to parse as TimelineObjectDefinition
            let parsed: Result<TimelineObjectDefinition, XmlError> = TimelineObjectDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformLicenseDefinition" => {
            // try to parse as PlatformLicenseDefinition
            let parsed: Result<PlatformLicenseDefinition, XmlError> = PlatformLicenseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FuelTypeSustnUom" => {
            // try to parse as FuelTypeSustnUom
            let parsed: Result<FuelTypeSustnUom, XmlError> = FuelTypeSustnUom::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NotificationChannels" => {
            // try to parse as NotificationChannels
            let parsed: Result<NotificationChannels, XmlError> = NotificationChannels::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiLocalPlugin" => {
            // try to parse as GenAiLocalPlugin
            let parsed: Result<GenAiLocalPlugin, XmlError> = GenAiLocalPlugin::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowCustomErrorMessageTranslation" => {
            // try to parse as FlowCustomErrorMessageTranslation
            let parsed: Result<FlowCustomErrorMessageTranslation, XmlError> = FlowCustomErrorMessageTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordAggregationDefinition" => {
            // try to parse as RecordAggregationDefinition
            let parsed: Result<RecordAggregationDefinition, XmlError> = RecordAggregationDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DisclosureDefinition" => {
            // try to parse as DisclosureDefinition
            let parsed: Result<DisclosureDefinition, XmlError> = DisclosureDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ArticleTypeChannelDisplay" => {
            // try to parse as ArticleTypeChannelDisplay
            let parsed: Result<ArticleTypeChannelDisplay, XmlError> = ArticleTypeChannelDisplay::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MobileSettings" => {
            // try to parse as MobileSettings
            let parsed: Result<MobileSettings, XmlError> = MobileSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportDataCategoryFilter" => {
            // try to parse as ReportDataCategoryFilter
            let parsed: Result<ReportDataCategoryFilter, XmlError> = ReportDataCategoryFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomAddressFieldSettings" => {
            // try to parse as CustomAddressFieldSettings
            let parsed: Result<CustomAddressFieldSettings, XmlError> = CustomAddressFieldSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceSetupAssistantSettings" => {
            // try to parse as ServiceSetupAssistantSettings
            let parsed: Result<ServiceSetupAssistantSettings, XmlError> = ServiceSetupAssistantSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContentAssetRelationships" => {
            // try to parse as ContentAssetRelationships
            let parsed: Result<ContentAssetRelationships, XmlError> = ContentAssetRelationships::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SidebarComponent" => {
            // try to parse as SidebarComponent
            let parsed: Result<SidebarComponent, XmlError> = SidebarComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppComponentList" => {
            // try to parse as AppComponentList
            let parsed: Result<AppComponentList, XmlError> = AppComponentList::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppPreferences" => {
            // try to parse as AppPreferences
            let parsed: Result<AppPreferences, XmlError> = AppPreferences::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformEventChannel" => {
            // try to parse as PlatformEventChannel
            let parsed: Result<PlatformEventChannel, XmlError> = PlatformEventChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "HerokuAppLinkSettings" => {
            // try to parse as HerokuAppLinkSettings
            let parsed: Result<HerokuAppLinkSettings, XmlError> = HerokuAppLinkSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataConnection" => {
            // try to parse as MktDataConnection
            let parsed: Result<MktDataConnection, XmlError> = MktDataConnection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccessMapping" => {
            // try to parse as AccessMapping
            let parsed: Result<AccessMapping, XmlError> = AccessMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobAtomicWritebackRelationship" => {
            // try to parse as BatchCalcJobAtomicWritebackRelationship
            let parsed: Result<BatchCalcJobAtomicWritebackRelationship, XmlError> = BatchCalcJobAtomicWritebackRelationship::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PricingRecipeTableMapping" => {
            // try to parse as PricingRecipeTableMapping
            let parsed: Result<PricingRecipeTableMapping, XmlError> = PricingRecipeTableMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ScheduledRecommendationDetail" => {
            // try to parse as ScheduledRecommendationDetail
            let parsed: Result<ScheduledRecommendationDetail, XmlError> = ScheduledRecommendationDetail::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharingSettings" => {
            // try to parse as SharingSettings
            let parsed: Result<SharingSettings, XmlError> = SharingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTask" => {
            // try to parse as EnablementProgramTask
            let parsed: Result<EnablementProgramTask, XmlError> = EnablementProgramTask::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Keyword" => {
            // try to parse as Keyword
            let parsed: Result<Keyword, XmlError> = Keyword::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobTransformDroppedField" => {
            // try to parse as BatchCalcJobTransformDroppedField
            let parsed: Result<BatchCalcJobTransformDroppedField, XmlError> = BatchCalcJobTransformDroppedField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TrailheadSettings" => {
            // try to parse as TrailheadSettings
            let parsed: Result<TrailheadSettings, XmlError> = TrailheadSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NtfcnChannelCont" => {
            // try to parse as NtfcnChannelCont
            let parsed: Result<NtfcnChannelCont, XmlError> = NtfcnChannelCont::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcessRuleStepMapping" => {
            // try to parse as LoyaltyProgramProcessRuleStepMapping
            let parsed: Result<LoyaltyProgramProcessRuleStepMapping, XmlError> = LoyaltyProgramProcessRuleStepMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveMessageSettings" => {
            // try to parse as LiveMessageSettings
            let parsed: Result<LiveMessageSettings, XmlError> = LiveMessageSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NetworkPageOverride" => {
            // try to parse as NetworkPageOverride
            let parsed: Result<NetworkPageOverride, XmlError> = NetworkPageOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CanvasMetadata" => {
            // try to parse as CanvasMetadata
            let parsed: Result<CanvasMetadata, XmlError> = CanvasMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StgFulfillmentStepDef" => {
            // try to parse as StgFulfillmentStepDef
            let parsed: Result<StgFulfillmentStepDef, XmlError> = StgFulfillmentStepDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileFieldLevelSecurity" => {
            // try to parse as ProfileFieldLevelSecurity
            let parsed: Result<ProfileFieldLevelSecurity, XmlError> = ProfileFieldLevelSecurity::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorAttributeOpt" => {
            // try to parse as DataConnectorAttributeOpt
            let parsed: Result<DataConnectorAttributeOpt, XmlError> = DataConnectorAttributeOpt::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrTemplateSampleDocument" => {
            // try to parse as OcrTemplateSampleDocument
            let parsed: Result<OcrTemplateSampleDocument, XmlError> = OcrTemplateSampleDocument::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAccountForecastSet" => {
            // try to parse as AdvAccountForecastSet
            let parsed: Result<AdvAccountForecastSet, XmlError> = AdvAccountForecastSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedListItem" => {
            // try to parse as RelatedListItem
            let parsed: Result<RelatedListItem, XmlError> = RelatedListItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetEmailRoutingAddressAccess" => {
            // try to parse as PermissionSetEmailRoutingAddressAccess
            let parsed: Result<PermissionSetEmailRoutingAddressAccess, XmlError> = PermissionSetEmailRoutingAddressAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryModelCard" => {
            // try to parse as DiscoveryModelCard
            let parsed: Result<DiscoveryModelCard, XmlError> = DiscoveryModelCard::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsVizViewDef" => {
            // try to parse as AnalyticsVizViewDef
            let parsed: Result<AnalyticsVizViewDef, XmlError> = AnalyticsVizViewDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChannelObjectLinkingRule" => {
            // try to parse as ChannelObjectLinkingRule
            let parsed: Result<ChannelObjectLinkingRule, XmlError> = ChannelObjectLinkingRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LeadConvertSettings" => {
            // try to parse as LeadConvertSettings
            let parsed: Result<LeadConvertSettings, XmlError> = LeadConvertSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventSettings" => {
            // try to parse as EventSettings
            let parsed: Result<EventSettings, XmlError> = EventSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationSystemMessageMapping" => {
            // try to parse as ConversationSystemMessageMapping
            let parsed: Result<ConversationSystemMessageMapping, XmlError> = ConversationSystemMessageMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PublicKeyCertificateSet" => {
            // try to parse as PublicKeyCertificateSet
            let parsed: Result<PublicKeyCertificateSet, XmlError> = PublicKeyCertificateSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestTopLevelWithDeclMd3" => {
            // try to parse as FtestTopLevelWithDeclMd3
            let parsed: Result<FtestTopLevelWithDeclMd3, XmlError> = FtestTopLevelWithDeclMd3::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryModelTransform" => {
            // try to parse as DiscoveryModelTransform
            let parsed: Result<DiscoveryModelTransform, XmlError> = DiscoveryModelTransform::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchCustomizationRuleValue" => {
            // try to parse as SearchCustomizationRuleValue
            let parsed: Result<SearchCustomizationRuleValue, XmlError> = SearchCustomizationRuleValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesRatingSettings" => {
            // try to parse as IndustriesRatingSettings
            let parsed: Result<IndustriesRatingSettings, XmlError> = IndustriesRatingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DeploymentSettings" => {
            // try to parse as DeploymentSettings
            let parsed: Result<DeploymentSettings, XmlError> = DeploymentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnhancedNotesSettings" => {
            // try to parse as EnhancedNotesSettings
            let parsed: Result<EnhancedNotesSettings, XmlError> = EnhancedNotesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryFieldMap" => {
            // try to parse as DiscoveryFieldMap
            let parsed: Result<DiscoveryFieldMap, XmlError> = DiscoveryFieldMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PolicyRuleDefinitionClauseConjunction" => {
            // try to parse as PolicyRuleDefinitionClauseConjunction
            let parsed: Result<PolicyRuleDefinitionClauseConjunction, XmlError> = PolicyRuleDefinitionClauseConjunction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApplicationSubtypeDefinition" => {
            // try to parse as ApplicationSubtypeDefinition
            let parsed: Result<ApplicationSubtypeDefinition, XmlError> = ApplicationSubtypeDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomField" => {
            // try to parse as CustomField
            let parsed: Result<CustomField, XmlError> = CustomField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppOauthPolicy" => {
            // try to parse as ConnectedAppOauthPolicy
            let parsed: Result<ConnectedAppOauthPolicy, XmlError> = ConnectedAppOauthPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppIpRange" => {
            // try to parse as ConnectedAppIpRange
            let parsed: Result<ConnectedAppIpRange, XmlError> = ConnectedAppIpRange::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceAISetupField" => {
            // try to parse as ServiceAISetupField
            let parsed: Result<ServiceAISetupField, XmlError> = ServiceAISetupField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SrvcMgmtObjCollabAppCnfg" => {
            // try to parse as SrvcMgmtObjCollabAppCnfg
            let parsed: Result<SrvcMgmtObjCollabAppCnfg, XmlError> = SrvcMgmtObjCollabAppCnfg::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Audience" => {
            // try to parse as Audience
            let parsed: Result<Audience, XmlError> = Audience::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalStep" => {
            // try to parse as ApprovalStep
            let parsed: Result<ApprovalStep, XmlError> = ApprovalStep::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageEventPropertyMapping" => {
            // try to parse as FlexiPageEventPropertyMapping
            let parsed: Result<FlexiPageEventPropertyMapping, XmlError> = FlexiPageEventPropertyMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiPlannerVoiceDef" => {
            // try to parse as AiPlannerVoiceDef
            let parsed: Result<AiPlannerVoiceDef, XmlError> = AiPlannerVoiceDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TelemetryActionDefinition" => {
            // try to parse as TelemetryActionDefinition
            let parsed: Result<TelemetryActionDefinition, XmlError> = TelemetryActionDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AuthProvider" => {
            // try to parse as AuthProvider
            let parsed: Result<AuthProvider, XmlError> = AuthProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductSettings" => {
            // try to parse as ProductSettings
            let parsed: Result<ProductSettings, XmlError> = ProductSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTaskExercise" => {
            // try to parse as EnablementProgramTaskExercise
            let parsed: Result<EnablementProgramTaskExercise, XmlError> = EnablementProgramTaskExercise::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportBlockInfo" => {
            // try to parse as ReportBlockInfo
            let parsed: Result<ReportBlockInfo, XmlError> = ReportBlockInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfigUser" => {
            // try to parse as OmniSupervisorConfigUser
            let parsed: Result<OmniSupervisorConfigUser, XmlError> = OmniSupervisorConfigUser::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActvPlatformFieldValue" => {
            // try to parse as ActvPlatformFieldValue
            let parsed: Result<ActvPlatformFieldValue, XmlError> = ActvPlatformFieldValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotTranslation" => {
            // try to parse as BotTranslation
            let parsed: Result<BotTranslation, XmlError> = BotTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SynonymGroup" => {
            // try to parse as SynonymGroup
            let parsed: Result<SynonymGroup, XmlError> = SynonymGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdDimension" => {
            // try to parse as WaveXmdDimension
            let parsed: Result<WaveXmdDimension, XmlError> = WaveXmdDimension::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedContent" => {
            // try to parse as RelatedContent
            let parsed: Result<RelatedContent, XmlError> = RelatedContent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomObject" => {
            // try to parse as CustomObject
            let parsed: Result<CustomObject, XmlError> = CustomObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContentTypeBundleResource" => {
            // try to parse as ContentTypeBundleResource
            let parsed: Result<ContentTypeBundleResource, XmlError> = ContentTypeBundleResource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticSnapshot" => {
            // try to parse as AnalyticSnapshot
            let parsed: Result<AnalyticSnapshot, XmlError> = AnalyticSnapshot::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesConnectedServiceSettings" => {
            // try to parse as IndustriesConnectedServiceSettings
            let parsed: Result<IndustriesConnectedServiceSettings, XmlError> = IndustriesConnectedServiceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StandardPermissionSet" => {
            // try to parse as StandardPermissionSet
            let parsed: Result<StandardPermissionSet, XmlError> = StandardPermissionSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotTemplateTranslation" => {
            // try to parse as BotTemplateTranslation
            let parsed: Result<BotTemplateTranslation, XmlError> = BotTemplateTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FundraisingConfig" => {
            // try to parse as FundraisingConfig
            let parsed: Result<FundraisingConfig, XmlError> = FundraisingConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingObjectListUnselectedSettings" => {
            // try to parse as ForecastingObjectListUnselectedSettings
            let parsed: Result<ForecastingObjectListUnselectedSettings, XmlError> = ForecastingObjectListUnselectedSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BusinessProcessFeedback" => {
            // try to parse as BusinessProcessFeedback
            let parsed: Result<BusinessProcessFeedback, XmlError> = BusinessProcessFeedback::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkflowTimeTrigger" => {
            // try to parse as WorkflowTimeTrigger
            let parsed: Result<WorkflowTimeTrigger, XmlError> = WorkflowTimeTrigger::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RunTestSuccess" => {
            // try to parse as RunTestSuccess
            let parsed: Result<RunTestSuccess, XmlError> = RunTestSuccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataspaceScopeAccess" => {
            // try to parse as DataspaceScopeAccess
            let parsed: Result<DataspaceScopeAccess, XmlError> = DataspaceScopeAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdMeasure" => {
            // try to parse as WaveXmdMeasure
            let parsed: Result<WaveXmdMeasure, XmlError> = WaveXmdMeasure::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ValidationRuleTranslation" => {
            // try to parse as ValidationRuleTranslation
            let parsed: Result<ValidationRuleTranslation, XmlError> = ValidationRuleTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuoteSettings" => {
            // try to parse as QuoteSettings
            let parsed: Result<QuoteSettings, XmlError> = QuoteSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmailServicesFunction" => {
            // try to parse as EmailServicesFunction
            let parsed: Result<EmailServicesFunction, XmlError> = EmailServicesFunction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkSkillRouting" => {
            // try to parse as WorkSkillRouting
            let parsed: Result<WorkSkillRouting, XmlError> = WorkSkillRouting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeSitesSettings" => {
            // try to parse as KnowledgeSitesSettings
            let parsed: Result<KnowledgeSitesSettings, XmlError> = KnowledgeSitesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalDataSrcDescriptor" => {
            // try to parse as ExternalDataSrcDescriptor
            let parsed: Result<ExternalDataSrcDescriptor, XmlError> = ExternalDataSrcDescriptor::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Icon" => {
            // try to parse as Icon
            let parsed: Result<Icon, XmlError> = Icon::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MLFilter" => {
            // try to parse as MLFilter
            let parsed: Result<MLFilter, XmlError> = MLFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppMenuItem" => {
            // try to parse as AppMenuItem
            let parsed: Result<AppMenuItem, XmlError> = AppMenuItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StgFulfillmentStepDefGrp" => {
            // try to parse as StgFulfillmentStepDefGrp
            let parsed: Result<StgFulfillmentStepDefGrp, XmlError> = StgFulfillmentStepDefGrp::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecAlrtDataSrcExpSetDef" => {
            // try to parse as RecAlrtDataSrcExpSetDef
            let parsed: Result<RecAlrtDataSrcExpSetDef, XmlError> = RecAlrtDataSrcExpSetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SiteIframeWhiteListUrl" => {
            // try to parse as SiteIframeWhiteListUrl
            let parsed: Result<SiteIframeWhiteListUrl, XmlError> = SiteIframeWhiteListUrl::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsDashboardLayout" => {
            // try to parse as AnalyticsDashboardLayout
            let parsed: Result<AnalyticsDashboardLayout, XmlError> = AnalyticsDashboardLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataImportManagementSettings" => {
            // try to parse as DataImportManagementSettings
            let parsed: Result<DataImportManagementSettings, XmlError> = DataImportManagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAccountForecastPeriod" => {
            // try to parse as AdvAccountForecastPeriod
            let parsed: Result<AdvAccountForecastPeriod, XmlError> = AdvAccountForecastPeriod::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WindowsPushApplicationSetup" => {
            // try to parse as WindowsPushApplicationSetup
            let parsed: Result<WindowsPushApplicationSetup, XmlError> = WindowsPushApplicationSetup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FiscalYearSettings" => {
            // try to parse as FiscalYearSettings
            let parsed: Result<FiscalYearSettings, XmlError> = FiscalYearSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchDataSrcFilterCriteria" => {
            // try to parse as BatchDataSrcFilterCriteria
            let parsed: Result<BatchDataSrcFilterCriteria, XmlError> = BatchDataSrcFilterCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotMessage" => {
            // try to parse as BotMessage
            let parsed: Result<BotMessage, XmlError> = BotMessage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DynamicFormsSettings" => {
            // try to parse as DynamicFormsSettings
            let parsed: Result<DynamicFormsSettings, XmlError> = DynamicFormsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SkillUserAssignments" => {
            // try to parse as SkillUserAssignments
            let parsed: Result<SkillUserAssignments, XmlError> = SkillUserAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageConstantPrimitiveValue" => {
            // try to parse as ConversationMessageConstantPrimitiveValue
            let parsed: Result<ConversationMessageConstantPrimitiveValue, XmlError> = ConversationMessageConstantPrimitiveValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FulfillmentStepType" => {
            // try to parse as FulfillmentStepType
            let parsed: Result<FulfillmentStepType, XmlError> = FulfillmentStepType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Community" => {
            // try to parse as Community
            let parsed: Result<Community, XmlError> = Community::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomFeedFilter" => {
            // try to parse as CustomFeedFilter
            let parsed: Result<CustomFeedFilter, XmlError> = CustomFeedFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RunTestFailure" => {
            // try to parse as RunTestFailure
            let parsed: Result<RunTestFailure, XmlError> = RunTestFailure::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestTopLevelWithDeclMd1" => {
            // try to parse as FtestTopLevelWithDeclMd1
            let parsed: Result<FtestTopLevelWithDeclMd1, XmlError> = FtestTopLevelWithDeclMd1::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningOutApp" => {
            // try to parse as LightningOutApp
            let parsed: Result<LightningOutApp, XmlError> = LightningOutApp::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EinsteinAgentSettings" => {
            // try to parse as EinsteinAgentSettings
            let parsed: Result<EinsteinAgentSettings, XmlError> = EinsteinAgentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MediaAdSalesSettings" => {
            // try to parse as MediaAdSalesSettings
            let parsed: Result<MediaAdSalesSettings, XmlError> = MediaAdSalesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataKitObjectDependency" => {
            // try to parse as DataKitObjectDependency
            let parsed: Result<DataKitObjectDependency, XmlError> = DataKitObjectDependency::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Prompt" => {
            // try to parse as Prompt
            let parsed: Result<Prompt, XmlError> = Prompt::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportChartComponentLayoutItem" => {
            // try to parse as ReportChartComponentLayoutItem
            let parsed: Result<ReportChartComponentLayoutItem, XmlError> = ReportChartComponentLayoutItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SettingItem" => {
            // try to parse as SettingItem
            let parsed: Result<SettingItem, XmlError> = SettingItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NavigationLinkSet" => {
            // try to parse as NavigationLinkSet
            let parsed: Result<NavigationLinkSet, XmlError> = NavigationLinkSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SchedulingRuleParameter" => {
            // try to parse as SchedulingRuleParameter
            let parsed: Result<SchedulingRuleParameter, XmlError> = SchedulingRuleParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobJoinResultField" => {
            // try to parse as BatchCalcJobJoinResultField
            let parsed: Result<BatchCalcJobJoinResultField, XmlError> = BatchCalcJobJoinResultField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppPushSettings" => {
            // try to parse as ExtlClntAppPushSettings
            let parsed: Result<ExtlClntAppPushSettings, XmlError> = ExtlClntAppPushSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PresenceConfigUserAssignments" => {
            // try to parse as PresenceConfigUserAssignments
            let parsed: Result<PresenceConfigUserAssignments, XmlError> = PresenceConfigUserAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ValueTypeField" => {
            // try to parse as ValueTypeField
            let parsed: Result<ValueTypeField, XmlError> = ValueTypeField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtendedErrorDetails" => {
            // try to parse as ExtendedErrorDetails
            let parsed: Result<ExtendedErrorDetails, XmlError> = ExtendedErrorDetails::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticSnapshotMapping" => {
            // try to parse as AnalyticSnapshotMapping
            let parsed: Result<AnalyticSnapshotMapping, XmlError> = AnalyticSnapshotMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPromptTemplate" => {
            // try to parse as GenAiPromptTemplate
            let parsed: Result<GenAiPromptTemplate, XmlError> = GenAiPromptTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Web3Settings" => {
            // try to parse as Web3Settings
            let parsed: Result<Web3Settings, XmlError> = Web3Settings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NetworkEmailTmplAllowlist" => {
            // try to parse as NetworkEmailTmplAllowlist
            let parsed: Result<NetworkEmailTmplAllowlist, XmlError> = NetworkEmailTmplAllowlist::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextAttributeMapping" => {
            // try to parse as ContextAttributeMapping
            let parsed: Result<ContextAttributeMapping, XmlError> = ContextAttributeMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalDataTranObject" => {
            // try to parse as ExternalDataTranObject
            let parsed: Result<ExternalDataTranObject, XmlError> = ExternalDataTranObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotStepCondition" => {
            // try to parse as BotStepCondition
            let parsed: Result<BotStepCondition, XmlError> = BotStepCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchCustomizationRule" => {
            // try to parse as SearchCustomizationRule
            let parsed: Result<SearchCustomizationRule, XmlError> = SearchCustomizationRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConvIntelligenceSignalSubRule" => {
            // try to parse as ConvIntelligenceSignalSubRule
            let parsed: Result<ConvIntelligenceSignalSubRule, XmlError> = ConvIntelligenceSignalSubRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2Type" => {
            // try to parse as Territory2Type
            let parsed: Result<Territory2Type, XmlError> = Territory2Type::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LearningItemType" => {
            // try to parse as LearningItemType
            let parsed: Result<LearningItemType, XmlError> = LearningItemType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CampaignSettings" => {
            // try to parse as CampaignSettings
            let parsed: Result<CampaignSettings, XmlError> = CampaignSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StgAssignmentRuleCond" => {
            // try to parse as StgAssignmentRuleCond
            let parsed: Result<StgAssignmentRuleCond, XmlError> = StgAssignmentRuleCond::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VoiceEngagementMediaUsage" => {
            // try to parse as VoiceEngagementMediaUsage
            let parsed: Result<VoiceEngagementMediaUsage, XmlError> = VoiceEngagementMediaUsage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeedLayoutFilter" => {
            // try to parse as FeedLayoutFilter
            let parsed: Result<FeedLayoutFilter, XmlError> = FeedLayoutFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssignmentRules" => {
            // try to parse as AssignmentRules
            let parsed: Result<AssignmentRules, XmlError> = AssignmentRules::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QueueRoutingConfig" => {
            // try to parse as QueueRoutingConfig
            let parsed: Result<QueueRoutingConfig, XmlError> = QueueRoutingConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingChannel" => {
            // try to parse as MessagingChannel
            let parsed: Result<MessagingChannel, XmlError> = MessagingChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommsServiceConsoleSettings" => {
            // try to parse as CommsServiceConsoleSettings
            let parsed: Result<CommsServiceConsoleSettings, XmlError> = CommsServiceConsoleSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldServiceMobileConfig" => {
            // try to parse as FieldServiceMobileConfig
            let parsed: Result<FieldServiceMobileConfig, XmlError> = FieldServiceMobileConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OrchestrationContextEvent" => {
            // try to parse as OrchestrationContextEvent
            let parsed: Result<OrchestrationContextEvent, XmlError> = OrchestrationContextEvent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "McpServerToolApiDefinition" => {
            // try to parse as McpServerToolApiDefinition
            let parsed: Result<McpServerToolApiDefinition, XmlError> = McpServerToolApiDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppNotificationType" => {
            // try to parse as AppNotificationType
            let parsed: Result<AppNotificationType, XmlError> = AppNotificationType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceCloudVoiceSettings" => {
            // try to parse as ServiceCloudVoiceSettings
            let parsed: Result<ServiceCloudVoiceSettings, XmlError> = ServiceCloudVoiceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BenefitActionParameter" => {
            // try to parse as BenefitActionParameter
            let parsed: Result<BenefitActionParameter, XmlError> = BenefitActionParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIDataDefinition" => {
            // try to parse as AIDataDefinition
            let parsed: Result<AIDataDefinition, XmlError> = AIDataDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LaborCostOptimizationSettings" => {
            // try to parse as LaborCostOptimizationSettings
            let parsed: Result<LaborCostOptimizationSettings, XmlError> = LaborCostOptimizationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FunctionReference" => {
            // try to parse as FunctionReference
            let parsed: Result<FunctionReference, XmlError> = FunctionReference::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageConstantCompositeValue" => {
            // try to parse as ConversationMessageConstantCompositeValue
            let parsed: Result<ConversationMessageConstantCompositeValue, XmlError> = ConversationMessageConstantCompositeValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RoleAndSubordinatesInternal" => {
            // try to parse as RoleAndSubordinatesInternal
            let parsed: Result<RoleAndSubordinatesInternal, XmlError> = RoleAndSubordinatesInternal::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountForecastFormula" => {
            // try to parse as AccountForecastFormula
            let parsed: Result<AccountForecastFormula, XmlError> = AccountForecastFormula::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlannerAttrMapping" => {
            // try to parse as GenAiPlannerAttrMapping
            let parsed: Result<GenAiPlannerAttrMapping, XmlError> = GenAiPlannerAttrMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReferencedDashboard" => {
            // try to parse as ReferencedDashboard
            let parsed: Result<ReferencedDashboard, XmlError> = ReferencedDashboard::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkforceEngagementSettings" => {
            // try to parse as WorkforceEngagementSettings
            let parsed: Result<WorkforceEngagementSettings, XmlError> = WorkforceEngagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataObjectCategory" => {
            // try to parse as DataObjectCategory
            let parsed: Result<DataObjectCategory, XmlError> = DataObjectCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharingReasonTranslation" => {
            // try to parse as SharingReasonTranslation
            let parsed: Result<SharingReasonTranslation, XmlError> = SharingReasonTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UpsertResult" => {
            // try to parse as UpsertResult
            let parsed: Result<UpsertResult, XmlError> = UpsertResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileServicePresenceStatusAccess" => {
            // try to parse as ProfileServicePresenceStatusAccess
            let parsed: Result<ProfileServicePresenceStatusAccess, XmlError> = ProfileServicePresenceStatusAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIPredictionExpression" => {
            // try to parse as AIPredictionExpression
            let parsed: Result<AIPredictionExpression, XmlError> = AIPredictionExpression::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceQuickAction" => {
            // try to parse as EmbeddedServiceQuickAction
            let parsed: Result<EmbeddedServiceQuickAction, XmlError> = EmbeddedServiceQuickAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeatureParameterInteger" => {
            // try to parse as FeatureParameterInteger
            let parsed: Result<FeatureParameterInteger, XmlError> = FeatureParameterInteger::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcessConditionFilterCriteria" => {
            // try to parse as LoyaltyProgramProcessConditionFilterCriteria
            let parsed: Result<LoyaltyProgramProcessConditionFilterCriteria, XmlError> = LoyaltyProgramProcessConditionFilterCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PersonalizationTargetInfos" => {
            // try to parse as PersonalizationTargetInfos
            let parsed: Result<PersonalizationTargetInfos, XmlError> = PersonalizationTargetInfos::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataSourceObject" => {
            // try to parse as DataSourceObject
            let parsed: Result<DataSourceObject, XmlError> = DataSourceObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantDefinition" => {
            // try to parse as AssistantDefinition
            let parsed: Result<AssistantDefinition, XmlError> = AssistantDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UiFormatSpecificationSet" => {
            // try to parse as UiFormatSpecificationSet
            let parsed: Result<UiFormatSpecificationSet, XmlError> = UiFormatSpecificationSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VoiceEngagementMediaFile" => {
            // try to parse as VoiceEngagementMediaFile
            let parsed: Result<VoiceEngagementMediaFile, XmlError> = VoiceEngagementMediaFile::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NetworkAccess" => {
            // try to parse as NetworkAccess
            let parsed: Result<NetworkAccess, XmlError> = NetworkAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConvReasonReportSegmentDef" => {
            // try to parse as ConvReasonReportSegmentDef
            let parsed: Result<ConvReasonReportSegmentDef, XmlError> = ConvReasonReportSegmentDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotTemplate" => {
            // try to parse as BotTemplate
            let parsed: Result<BotTemplate, XmlError> = BotTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileUserPermission" => {
            // try to parse as ProfileUserPermission
            let parsed: Result<ProfileUserPermission, XmlError> = ProfileUserPermission::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FindSimilarOppFilter" => {
            // try to parse as FindSimilarOppFilter
            let parsed: Result<FindSimilarOppFilter, XmlError> = FindSimilarOppFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningBoltItems" => {
            // try to parse as LightningBoltItems
            let parsed: Result<LightningBoltItems, XmlError> = LightningBoltItems::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StgAssignmentRuleCriteria" => {
            // try to parse as StgAssignmentRuleCriteria
            let parsed: Result<StgAssignmentRuleCriteria, XmlError> = StgAssignmentRuleCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedContentItem" => {
            // try to parse as RelatedContentItem
            let parsed: Result<RelatedContentItem, XmlError> = RelatedContentItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAcctFrcstDisplayGroup" => {
            // try to parse as AdvAcctFrcstDisplayGroup
            let parsed: Result<AdvAcctFrcstDisplayGroup, XmlError> = AdvAcctFrcstDisplayGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BlockchainSettings" => {
            // try to parse as BlockchainSettings
            let parsed: Result<BlockchainSettings, XmlError> = BlockchainSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PrcShtAttrDefinition" => {
            // try to parse as PrcShtAttrDefinition
            let parsed: Result<PrcShtAttrDefinition, XmlError> = PrcShtAttrDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InboundNetworkConnection" => {
            // try to parse as InboundNetworkConnection
            let parsed: Result<InboundNetworkConnection, XmlError> = InboundNetworkConnection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomPermission" => {
            // try to parse as CustomPermission
            let parsed: Result<CustomPermission, XmlError> = CustomPermission::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIReplyRecommendationsSettings" => {
            // try to parse as AIReplyRecommendationsSettings
            let parsed: Result<AIReplyRecommendationsSettings, XmlError> = AIReplyRecommendationsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EntitlementProcess" => {
            // try to parse as EntitlementProcess
            let parsed: Result<EntitlementProcess, XmlError> = EntitlementProcess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnimationRule" => {
            // try to parse as AnimationRule
            let parsed: Result<AnimationRule, XmlError> = AnimationRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BriefcaseDefinition" => {
            // try to parse as BriefcaseDefinition
            let parsed: Result<BriefcaseDefinition, XmlError> = BriefcaseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationRecordLookup" => {
            // try to parse as ConversationRecordLookup
            let parsed: Result<ConversationRecordLookup, XmlError> = ConversationRecordLookup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EssentialsSettings" => {
            // try to parse as EssentialsSettings
            let parsed: Result<EssentialsSettings, XmlError> = EssentialsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalClientAppSettings" => {
            // try to parse as ExternalClientAppSettings
            let parsed: Result<ExternalClientAppSettings, XmlError> = ExternalClientAppSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Targets" => {
            // try to parse as Targets
            let parsed: Result<Targets, XmlError> = Targets::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformEncryptionSettings" => {
            // try to parse as PlatformEncryptionSettings
            let parsed: Result<PlatformEncryptionSettings, XmlError> = PlatformEncryptionSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LayoutSection" => {
            // try to parse as LayoutSection
            let parsed: Result<LayoutSection, XmlError> = LayoutSection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PathAssistantStep" => {
            // try to parse as PathAssistantStep
            let parsed: Result<PathAssistantStep, XmlError> = PathAssistantStep::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetObjectAliasField" => {
            // try to parse as ExpressionSetObjectAliasField
            let parsed: Result<ExpressionSetObjectAliasField, XmlError> = ExpressionSetObjectAliasField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NtfcnChannelDef" => {
            // try to parse as NtfcnChannelDef
            let parsed: Result<NtfcnChannelDef, XmlError> = NtfcnChannelDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExplainabilityMsgTemplateFieldTranslation" => {
            // try to parse as ExplainabilityMsgTemplateFieldTranslation
            let parsed: Result<ExplainabilityMsgTemplateFieldTranslation, XmlError> = ExplainabilityMsgTemplateFieldTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlSlotClass" => {
            // try to parse as MlSlotClass
            let parsed: Result<MlSlotClass, XmlError> = MlSlotClass::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorErrorTranslation" => {
            // try to parse as DataConnectorErrorTranslation
            let parsed: Result<DataConnectorErrorTranslation, XmlError> = DataConnectorErrorTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PaymentsManagementEnabledSettings" => {
            // try to parse as PaymentsManagementEnabledSettings
            let parsed: Result<PaymentsManagementEnabledSettings, XmlError> = PaymentsManagementEnabledSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApplePushApplicationSetup" => {
            // try to parse as ApplePushApplicationSetup
            let parsed: Result<ApplePushApplicationSetup, XmlError> = ApplePushApplicationSetup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlParameterOverride" => {
            // try to parse as MlParameterOverride
            let parsed: Result<MlParameterOverride, XmlError> = MlParameterOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastRangeSettings" => {
            // try to parse as ForecastRangeSettings
            let parsed: Result<ForecastRangeSettings, XmlError> = ForecastRangeSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Capabilities" => {
            // try to parse as Capabilities
            let parsed: Result<Capabilities, XmlError> = Capabilities::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPluginFunctionDef" => {
            // try to parse as GenAiPluginFunctionDef
            let parsed: Result<GenAiPluginFunctionDef, XmlError> = GenAiPluginFunctionDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CampaignTemplateDefinition" => {
            // try to parse as CampaignTemplateDefinition
            let parsed: Result<CampaignTemplateDefinition, XmlError> = CampaignTemplateDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountPlanSettings" => {
            // try to parse as AccountPlanSettings
            let parsed: Result<AccountPlanSettings, XmlError> = AccountPlanSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataModelFieldAttributes" => {
            // try to parse as MktDataModelFieldAttributes
            let parsed: Result<MktDataModelFieldAttributes, XmlError> = MktDataModelFieldAttributes::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserAccessPolicyFilter" => {
            // try to parse as UserAccessPolicyFilter
            let parsed: Result<UserAccessPolicyFilter, XmlError> = UserAccessPolicyFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ShiftSegmentType" => {
            // try to parse as ShiftSegmentType
            let parsed: Result<ShiftSegmentType, XmlError> = ShiftSegmentType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiEvaluationExpectation" => {
            // try to parse as AiEvaluationExpectation
            let parsed: Result<AiEvaluationExpectation, XmlError> = AiEvaluationExpectation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceCustomComponent" => {
            // try to parse as EmbeddedServiceCustomComponent
            let parsed: Result<EmbeddedServiceCustomComponent, XmlError> = EmbeddedServiceCustomComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataTranObject" => {
            // try to parse as MktDataTranObject
            let parsed: Result<MktDataTranObject, XmlError> = MktDataTranObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReputationLevel" => {
            // try to parse as ReputationLevel
            let parsed: Result<ReputationLevel, XmlError> = ReputationLevel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveApplication" => {
            // try to parse as WaveApplication
            let parsed: Result<WaveApplication, XmlError> = WaveApplication::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkflowTaskTranslation" => {
            // try to parse as WorkflowTaskTranslation
            let parsed: Result<WorkflowTaskTranslation, XmlError> = WorkflowTaskTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldSet" => {
            // try to parse as FieldSet
            let parsed: Result<FieldSet, XmlError> = FieldSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DescribeMetadataObject" => {
            // try to parse as DescribeMetadataObject
            let parsed: Result<DescribeMetadataObject, XmlError> = DescribeMetadataObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveChatButtonDeployments" => {
            // try to parse as LiveChatButtonDeployments
            let parsed: Result<LiveChatButtonDeployments, XmlError> = LiveChatButtonDeployments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowMetadataValue" => {
            // try to parse as FlowMetadataValue
            let parsed: Result<FlowMetadataValue, XmlError> = FlowMetadataValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsWorkspace" => {
            // try to parse as AnalyticsWorkspace
            let parsed: Result<AnalyticsWorkspace, XmlError> = AnalyticsWorkspace::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningExperienceTheme" => {
            // try to parse as LightningExperienceTheme
            let parsed: Result<LightningExperienceTheme, XmlError> = LightningExperienceTheme::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2" => {
            // try to parse as Territory2
            let parsed: Result<Territory2, XmlError> = Territory2::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RetrievalSummaryDefinition" => {
            // try to parse as RetrievalSummaryDefinition
            let parsed: Result<RetrievalSummaryDefinition, XmlError> = RetrievalSummaryDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "McpServerDefinition" => {
            // try to parse as McpServerDefinition
            let parsed: Result<McpServerDefinition, XmlError> = McpServerDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MyDomainSettings" => {
            // try to parse as MyDomainSettings
            let parsed: Result<MyDomainSettings, XmlError> = MyDomainSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Group" => {
            // try to parse as Group
            let parsed: Result<Group, XmlError> = Group::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InsuranceBrokerageSettings" => {
            // try to parse as InsuranceBrokerageSettings
            let parsed: Result<InsuranceBrokerageSettings, XmlError> = InsuranceBrokerageSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SalesWorkQueueSettings" => {
            // try to parse as SalesWorkQueueSettings
            let parsed: Result<SalesWorkQueueSettings, XmlError> = SalesWorkQueueSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndexField" => {
            // try to parse as IndexField
            let parsed: Result<IndexField, XmlError> = IndexField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApiNamedQuery" => {
            // try to parse as ApiNamedQuery
            let parsed: Result<ApiNamedQuery, XmlError> = ApiNamedQuery::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalProcess" => {
            // try to parse as ApprovalProcess
            let parsed: Result<ApprovalProcess, XmlError> = ApprovalProcess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniChannelPricingSettings" => {
            // try to parse as OmniChannelPricingSettings
            let parsed: Result<OmniChannelPricingSettings, XmlError> = OmniChannelPricingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetConditionCriteria" => {
            // try to parse as ExpressionSetConditionCriteria
            let parsed: Result<ExpressionSetConditionCriteria, XmlError> = ExpressionSetConditionCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogItemDef" => {
            // try to parse as SvcCatalogItemDef
            let parsed: Result<SvcCatalogItemDef, XmlError> = SvcCatalogItemDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformEventChannelMember" => {
            // try to parse as PlatformEventChannelMember
            let parsed: Result<PlatformEventChannelMember, XmlError> = PlatformEventChannelMember::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceProcessAttributeTranslation" => {
            // try to parse as ServiceProcessAttributeTranslation
            let parsed: Result<ServiceProcessAttributeTranslation, XmlError> = ServiceProcessAttributeTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationDefinitionNlpProvider" => {
            // try to parse as ConversationDefinitionNlpProvider
            let parsed: Result<ConversationDefinitionNlpProvider, XmlError> = ConversationDefinitionNlpProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalServiceOperation" => {
            // try to parse as ExternalServiceOperation
            let parsed: Result<ExternalServiceOperation, XmlError> = ExternalServiceOperation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppFrameworkTemplateBundle" => {
            // try to parse as AppFrameworkTemplateBundle
            let parsed: Result<AppFrameworkTemplateBundle, XmlError> = AppFrameworkTemplateBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductFamilyUsage" => {
            // try to parse as ProductFamilyUsage
            let parsed: Result<ProductFamilyUsage, XmlError> = ProductFamilyUsage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomIndex" => {
            // try to parse as CustomIndex
            let parsed: Result<CustomIndex, XmlError> = CustomIndex::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTaskFeedbackContent" => {
            // try to parse as EnablementProgramTaskFeedbackContent
            let parsed: Result<EnablementProgramTaskFeedbackContent, XmlError> = EnablementProgramTaskFeedbackContent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesManufacturingSettings" => {
            // try to parse as IndustriesManufacturingSettings
            let parsed: Result<IndustriesManufacturingSettings, XmlError> = IndustriesManufacturingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NamedCredential" => {
            // try to parse as NamedCredential
            let parsed: Result<NamedCredential, XmlError> = NamedCredential::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionableListDatasetColumn" => {
            // try to parse as ActionableListDatasetColumn
            let parsed: Result<ActionableListDatasetColumn, XmlError> = ActionableListDatasetColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIApplicationConfig" => {
            // try to parse as AIApplicationConfig
            let parsed: Result<AIApplicationConfig, XmlError> = AIApplicationConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VendorCallCenterStatusMap" => {
            // try to parse as VendorCallCenterStatusMap
            let parsed: Result<VendorCallCenterStatusMap, XmlError> = VendorCallCenterStatusMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TopLevelFTestMd1" => {
            // try to parse as TopLevelFTestMd1
            let parsed: Result<TopLevelFTestMd1, XmlError> = TopLevelFTestMd1::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionMatrixDefinitionVersionColumn" => {
            // try to parse as DecisionMatrixDefinitionVersionColumn
            let parsed: Result<DecisionMatrixDefinitionVersionColumn, XmlError> = DecisionMatrixDefinitionVersionColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ModuleRefs" => {
            // try to parse as ModuleRefs
            let parsed: Result<ModuleRefs, XmlError> = ModuleRefs::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsAssetAction" => {
            // try to parse as AnalyticsAssetAction
            let parsed: Result<AnalyticsAssetAction, XmlError> = AnalyticsAssetAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotQuickReplyOption" => {
            // try to parse as BotQuickReplyOption
            let parsed: Result<BotQuickReplyOption, XmlError> = BotQuickReplyOption::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfigAction" => {
            // try to parse as OmniSupervisorConfigAction
            let parsed: Result<OmniSupervisorConfigAction, XmlError> = OmniSupervisorConfigAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PriceSheetDefinition" => {
            // try to parse as PriceSheetDefinition
            let parsed: Result<PriceSheetDefinition, XmlError> = PriceSheetDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AcctMgrTargetSettings" => {
            // try to parse as AcctMgrTargetSettings
            let parsed: Result<AcctMgrTargetSettings, XmlError> = AcctMgrTargetSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeWorkOrderField" => {
            // try to parse as KnowledgeWorkOrderField
            let parsed: Result<KnowledgeWorkOrderField, XmlError> = KnowledgeWorkOrderField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowDefinition" => {
            // try to parse as FlowDefinition
            let parsed: Result<FlowDefinition, XmlError> = FlowDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RuleEntry" => {
            // try to parse as RuleEntry
            let parsed: Result<RuleEntry, XmlError> = RuleEntry::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomLabel" => {
            // try to parse as CustomLabel
            let parsed: Result<CustomLabel, XmlError> = CustomLabel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WebToCaseSettings" => {
            // try to parse as WebToCaseSettings
            let parsed: Result<WebToCaseSettings, XmlError> = WebToCaseSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdDimensionSalesforceAction" => {
            // try to parse as WaveXmdDimensionSalesforceAction
            let parsed: Result<WaveXmdDimensionSalesforceAction, XmlError> = WaveXmdDimensionSalesforceAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InvocableActionSettings" => {
            // try to parse as InvocableActionSettings
            let parsed: Result<InvocableActionSettings, XmlError> = InvocableActionSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppWorkspaceConfig" => {
            // try to parse as AppWorkspaceConfig
            let parsed: Result<AppWorkspaceConfig, XmlError> = AppWorkspaceConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectSearchSetting" => {
            // try to parse as ObjectSearchSetting
            let parsed: Result<ObjectSearchSetting, XmlError> = ObjectSearchSetting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CatalogedApiVersion" => {
            // try to parse as CatalogedApiVersion
            let parsed: Result<CatalogedApiVersion, XmlError> = CatalogedApiVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsButtonWidgetDef" => {
            // try to parse as AnalyticsButtonWidgetDef
            let parsed: Result<AnalyticsButtonWidgetDef, XmlError> = AnalyticsButtonWidgetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileApexPageAccess" => {
            // try to parse as ProfileApexPageAccess
            let parsed: Result<ProfileApexPageAccess, XmlError> = ProfileApexPageAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchResultActionConfig" => {
            // try to parse as SearchResultActionConfig
            let parsed: Result<SearchResultActionConfig, XmlError> = SearchResultActionConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlanner" => {
            // try to parse as GenAiPlanner
            let parsed: Result<GenAiPlanner, XmlError> = GenAiPlanner::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesSettings" => {
            // try to parse as IndustriesSettings
            let parsed: Result<IndustriesSettings, XmlError> = IndustriesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FldSvcBriefcaseRuleConfig" => {
            // try to parse as FldSvcBriefcaseRuleConfig
            let parsed: Result<FldSvcBriefcaseRuleConfig, XmlError> = FldSvcBriefcaseRuleConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantVersion" => {
            // try to parse as AssistantVersion
            let parsed: Result<AssistantVersion, XmlError> = AssistantVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RpaRobotPoolMetadata" => {
            // try to parse as RpaRobotPoolMetadata
            let parsed: Result<RpaRobotPoolMetadata, XmlError> = RpaRobotPoolMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActvPlatformAdncIdentifier" => {
            // try to parse as ActvPlatformAdncIdentifier
            let parsed: Result<ActvPlatformAdncIdentifier, XmlError> = ActvPlatformAdncIdentifier::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FuelType" => {
            // try to parse as FuelType
            let parsed: Result<FuelType, XmlError> = FuelType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppBrand" => {
            // try to parse as AppBrand
            let parsed: Result<AppBrand, XmlError> = AppBrand::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectNameCaseValue" => {
            // try to parse as ObjectNameCaseValue
            let parsed: Result<ObjectNameCaseValue, XmlError> = ObjectNameCaseValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MarketingAppExtActivity" => {
            // try to parse as MarketingAppExtActivity
            let parsed: Result<MarketingAppExtActivity, XmlError> = MarketingAppExtActivity::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SourceTrackingSettings" => {
            // try to parse as SourceTrackingSettings
            let parsed: Result<SourceTrackingSettings, XmlError> = SourceTrackingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationVendorInfo" => {
            // try to parse as ConversationVendorInfo
            let parsed: Result<ConversationVendorInfo, XmlError> = ConversationVendorInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AgentConfigAssignments" => {
            // try to parse as AgentConfigAssignments
            let parsed: Result<AgentConfigAssignments, XmlError> = AgentConfigAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationChannelDefinition" => {
            // try to parse as ConversationChannelDefinition
            let parsed: Result<ConversationChannelDefinition, XmlError> = ConversationChannelDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SynonymDictionary" => {
            // try to parse as SynonymDictionary
            let parsed: Result<SynonymDictionary, XmlError> = SynonymDictionary::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InternalOrganization" => {
            // try to parse as InternalOrganization
            let parsed: Result<InternalOrganization, XmlError> = InternalOrganization::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPromptTemplateVersion" => {
            // try to parse as GenAiPromptTemplateVersion
            let parsed: Result<GenAiPromptTemplateVersion, XmlError> = GenAiPromptTemplateVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Ai4mSettings" => {
            // try to parse as Ai4mSettings
            let parsed: Result<Ai4mSettings, XmlError> = Ai4mSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CatalogedApiArtifactVersionInfo" => {
            // try to parse as CatalogedApiArtifactVersionInfo
            let parsed: Result<CatalogedApiArtifactVersionInfo, XmlError> = CatalogedApiArtifactVersionInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ComponentInstanceProperty" => {
            // try to parse as ComponentInstanceProperty
            let parsed: Result<ComponentInstanceProperty, XmlError> = ComponentInstanceProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharingSet" => {
            // try to parse as SharingSet
            let parsed: Result<SharingSet, XmlError> = SharingSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomHttpHeader" => {
            // try to parse as CustomHttpHeader
            let parsed: Result<CustomHttpHeader, XmlError> = CustomHttpHeader::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnrichedField" => {
            // try to parse as EnrichedField
            let parsed: Result<EnrichedField, XmlError> = EnrichedField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SustainabilityUom" => {
            // try to parse as SustainabilityUom
            let parsed: Result<SustainabilityUom, XmlError> = SustainabilityUom::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionLayoutItem" => {
            // try to parse as QuickActionLayoutItem
            let parsed: Result<QuickActionLayoutItem, XmlError> = QuickActionLayoutItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextNodeAttrDictionary" => {
            // try to parse as ContextNodeAttrDictionary
            let parsed: Result<ContextNodeAttrDictionary, XmlError> = ContextNodeAttrDictionary::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CurrencySettings" => {
            // try to parse as CurrencySettings
            let parsed: Result<CurrencySettings, XmlError> = CurrencySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MailMergeSettings" => {
            // try to parse as MailMergeSettings
            let parsed: Result<MailMergeSettings, XmlError> = MailMergeSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MLField" => {
            // try to parse as MLField
            let parsed: Result<MLField, XmlError> = MLField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogFilterCondition" => {
            // try to parse as SvcCatalogFilterCondition
            let parsed: Result<SvcCatalogFilterCondition, XmlError> = SvcCatalogFilterCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIUsecaseModel" => {
            // try to parse as AIUsecaseModel
            let parsed: Result<AIUsecaseModel, XmlError> = AIUsecaseModel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmailTemplateSettings" => {
            // try to parse as EmailTemplateSettings
            let parsed: Result<EmailTemplateSettings, XmlError> = EmailTemplateSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenOpPlanEligibilityConfig" => {
            // try to parse as GenOpPlanEligibilityConfig
            let parsed: Result<GenOpPlanEligibilityConfig, XmlError> = GenOpPlanEligibilityConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfigQueue" => {
            // try to parse as OmniSupervisorConfigQueue
            let parsed: Result<OmniSupervisorConfigQueue, XmlError> = OmniSupervisorConfigQueue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomPageWebLinkTranslation" => {
            // try to parse as CustomPageWebLinkTranslation
            let parsed: Result<CustomPageWebLinkTranslation, XmlError> = CustomPageWebLinkTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OauthOidcSettings" => {
            // try to parse as OauthOidcSettings
            let parsed: Result<OauthOidcSettings, XmlError> = OauthOidcSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountIntelligenceSettings" => {
            // try to parse as AccountIntelligenceSettings
            let parsed: Result<AccountIntelligenceSettings, XmlError> = AccountIntelligenceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowOrchestrationStepTranslation" => {
            // try to parse as FlowOrchestrationStepTranslation
            let parsed: Result<FlowOrchestrationStepTranslation, XmlError> = FlowOrchestrationStepTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BusinessHoursSettings" => {
            // try to parse as BusinessHoursSettings
            let parsed: Result<BusinessHoursSettings, XmlError> = BusinessHoursSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobJoinKey" => {
            // try to parse as BatchCalcJobJoinKey
            let parsed: Result<BatchCalcJobJoinKey, XmlError> = BatchCalcJobJoinKey::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ModeratedEntityField" => {
            // try to parse as ModeratedEntityField
            let parsed: Result<ModeratedEntityField, XmlError> = ModeratedEntityField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DigitalExperienceConfig" => {
            // try to parse as DigitalExperienceConfig
            let parsed: Result<DigitalExperienceConfig, XmlError> = DigitalExperienceConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContentAssetVersions" => {
            // try to parse as ContentAssetVersions
            let parsed: Result<ContentAssetVersions, XmlError> = ContentAssetVersions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetVariableField" => {
            // try to parse as ExpressionSetVariableField
            let parsed: Result<ExpressionSetVariableField, XmlError> = ExpressionSetVariableField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InsPlcyLineOfBusConfig" => {
            // try to parse as InsPlcyLineOfBusConfig
            let parsed: Result<InsPlcyLineOfBusConfig, XmlError> = InsPlcyLineOfBusConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdDimensionMember" => {
            // try to parse as WaveXmdDimensionMember
            let parsed: Result<WaveXmdDimensionMember, XmlError> = WaveXmdDimensionMember::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomLabels" => {
            // try to parse as CustomLabels
            let parsed: Result<CustomLabels, XmlError> = CustomLabels::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotDialogTranslation" => {
            // try to parse as BotDialogTranslation
            let parsed: Result<BotDialogTranslation, XmlError> = BotDialogTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NtfcnChannelActionDef" => {
            // try to parse as NtfcnChannelActionDef
            let parsed: Result<NtfcnChannelActionDef, XmlError> = NtfcnChannelActionDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SecurityHubSettings" => {
            // try to parse as SecurityHubSettings
            let parsed: Result<SecurityHubSettings, XmlError> = SecurityHubSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PublicKeyCertificateSetKey" => {
            // try to parse as PublicKeyCertificateSetKey
            let parsed: Result<PublicKeyCertificateSetKey, XmlError> = PublicKeyCertificateSetKey::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportCustomDetailFormula" => {
            // try to parse as ReportCustomDetailFormula
            let parsed: Result<ReportCustomDetailFormula, XmlError> = ReportCustomDetailFormula::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AuthProvParamFwdAllowlist" => {
            // try to parse as AuthProvParamFwdAllowlist
            let parsed: Result<AuthProvParamFwdAllowlist, XmlError> = AuthProvParamFwdAllowlist::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NotificationsSettings" => {
            // try to parse as NotificationsSettings
            let parsed: Result<NotificationsSettings, XmlError> = NotificationsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetDefinitionVersion" => {
            // try to parse as ExpressionSetDefinitionVersion
            let parsed: Result<ExpressionSetDefinitionVersion, XmlError> = ExpressionSetDefinitionVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalStepApprover" => {
            // try to parse as ApprovalStepApprover
            let parsed: Result<ApprovalStepApprover, XmlError> = ApprovalStepApprover::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportType" => {
            // try to parse as ReportType
            let parsed: Result<ReportType, XmlError> = ReportType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionList" => {
            // try to parse as QuickActionList
            let parsed: Result<QuickActionList, XmlError> = QuickActionList::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ESignatureConfig" => {
            // try to parse as ESignatureConfig
            let parsed: Result<ESignatureConfig, XmlError> = ESignatureConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FTestToolingFLU" => {
            // try to parse as FTestToolingFLU
            let parsed: Result<FTestToolingFLU, XmlError> = FTestToolingFLU::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BusinessProcessGroup" => {
            // try to parse as BusinessProcessGroup
            let parsed: Result<BusinessProcessGroup, XmlError> = BusinessProcessGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageEvent" => {
            // try to parse as FlexiPageEvent
            let parsed: Result<FlexiPageEvent, XmlError> = FlexiPageEvent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingDisplayedFamilySettings" => {
            // try to parse as ForecastingDisplayedFamilySettings
            let parsed: Result<ForecastingDisplayedFamilySettings, XmlError> = ForecastingDisplayedFamilySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceConfig" => {
            // try to parse as EmbeddedServiceConfig
            let parsed: Result<EmbeddedServiceConfig, XmlError> = EmbeddedServiceConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PriceRuleCondition" => {
            // try to parse as PriceRuleCondition
            let parsed: Result<PriceRuleCondition, XmlError> = PriceRuleCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MatchingRule" => {
            // try to parse as MatchingRule
            let parsed: Result<MatchingRule, XmlError> = MatchingRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileSessionSetting" => {
            // try to parse as ProfileSessionSetting
            let parsed: Result<ProfileSessionSetting, XmlError> = ProfileSessionSetting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiPlannerSurfaceDef" => {
            // try to parse as AiPlannerSurfaceDef
            let parsed: Result<AiPlannerSurfaceDef, XmlError> = AiPlannerSurfaceDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportTimeFrameFilter" => {
            // try to parse as ReportTimeFrameFilter
            let parsed: Result<ReportTimeFrameFilter, XmlError> = ReportTimeFrameFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductCatalogManagementSettings" => {
            // try to parse as ProductCatalogManagementSettings
            let parsed: Result<ProductCatalogManagementSettings, XmlError> = ProductCatalogManagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardComponent" => {
            // try to parse as DashboardComponent
            let parsed: Result<DashboardComponent, XmlError> = DashboardComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContactCenterChannel" => {
            // try to parse as ContactCenterChannel
            let parsed: Result<ContactCenterChannel, XmlError> = ContactCenterChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "HighVelocitySalesSettings" => {
            // try to parse as HighVelocitySalesSettings
            let parsed: Result<HighVelocitySalesSettings, XmlError> = HighVelocitySalesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventSubscription" => {
            // try to parse as EventSubscription
            let parsed: Result<EventSubscription, XmlError> = EventSubscription::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SchedulingObjective" => {
            // try to parse as SchedulingObjective
            let parsed: Result<SchedulingObjective, XmlError> = SchedulingObjective::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EscalationRule" => {
            // try to parse as EscalationRule
            let parsed: Result<EscalationRule, XmlError> = EscalationRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AndroidPushApplicationSetup" => {
            // try to parse as AndroidPushApplicationSetup
            let parsed: Result<AndroidPushApplicationSetup, XmlError> = AndroidPushApplicationSetup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TagSet" => {
            // try to parse as TagSet
            let parsed: Result<TagSet, XmlError> = TagSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkspaceMapping" => {
            // try to parse as WorkspaceMapping
            let parsed: Result<WorkspaceMapping, XmlError> = WorkspaceMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceResource" => {
            // try to parse as EmbeddedServiceResource
            let parsed: Result<EmbeddedServiceResource, XmlError> = EmbeddedServiceResource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingTypeObjectListSettings" => {
            // try to parse as ForecastingTypeObjectListSettings
            let parsed: Result<ForecastingTypeObjectListSettings, XmlError> = ForecastingTypeObjectListSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TopicsForObjects" => {
            // try to parse as TopicsForObjects
            let parsed: Result<TopicsForObjects, XmlError> = TopicsForObjects::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTestReferenceOrValue" => {
            // try to parse as FlowTestReferenceOrValue
            let parsed: Result<FlowTestReferenceOrValue, XmlError> = FlowTestReferenceOrValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeatureParameterDate" => {
            // try to parse as FeatureParameterDate
            let parsed: Result<FeatureParameterDate, XmlError> = FeatureParameterDate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIScoringStep" => {
            // try to parse as AIScoringStep
            let parsed: Result<AIScoringStep, XmlError> = AIScoringStep::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorTranslation" => {
            // try to parse as DataConnectorTranslation
            let parsed: Result<DataConnectorTranslation, XmlError> = DataConnectorTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RoleAndSubordinates" => {
            // try to parse as RoleAndSubordinates
            let parsed: Result<RoleAndSubordinates, XmlError> = RoleAndSubordinates::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningMessageField" => {
            // try to parse as LightningMessageField
            let parsed: Result<LightningMessageField, XmlError> = LightningMessageField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveChatDeploymentDomainWhitelist" => {
            // try to parse as LiveChatDeploymentDomainWhitelist
            let parsed: Result<LiveChatDeploymentDomainWhitelist, XmlError> = LiveChatDeploymentDomainWhitelist::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IncludeEstTaxInQuoteSettings" => {
            // try to parse as IncludeEstTaxInQuoteSettings
            let parsed: Result<IncludeEstTaxInQuoteSettings, XmlError> = IncludeEstTaxInQuoteSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityAIModelMapping" => {
            // try to parse as CommunityAIModelMapping
            let parsed: Result<CommunityAIModelMapping, XmlError> = CommunityAIModelMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KeyboardShortcuts" => {
            // try to parse as KeyboardShortcuts
            let parsed: Result<KeyboardShortcuts, XmlError> = KeyboardShortcuts::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchCustomizationFieldOverride" => {
            // try to parse as SearchCustomizationFieldOverride
            let parsed: Result<SearchCustomizationFieldOverride, XmlError> = SearchCustomizationFieldOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmd" => {
            // try to parse as WaveXmd
            let parsed: Result<WaveXmd, XmlError> = WaveXmd::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExplainabilityActionDefinition" => {
            // try to parse as ExplainabilityActionDefinition
            let parsed: Result<ExplainabilityActionDefinition, XmlError> = ExplainabilityActionDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DuplicateRule" => {
            // try to parse as DuplicateRule
            let parsed: Result<DuplicateRule, XmlError> = DuplicateRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingTypeSettings" => {
            // try to parse as ForecastingTypeSettings
            let parsed: Result<ForecastingTypeSettings, XmlError> = ForecastingTypeSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ScheduledRecommendation" => {
            // try to parse as ScheduledRecommendation
            let parsed: Result<ScheduledRecommendation, XmlError> = ScheduledRecommendation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrSampleDocumentPage" => {
            // try to parse as OcrSampleDocumentPage
            let parsed: Result<OcrSampleDocumentPage, XmlError> = OcrSampleDocumentPage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeSettings" => {
            // try to parse as KnowledgeSettings
            let parsed: Result<KnowledgeSettings, XmlError> = KnowledgeSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileCustomSettingAccess" => {
            // try to parse as ProfileCustomSettingAccess
            let parsed: Result<ProfileCustomSettingAccess, XmlError> = ProfileCustomSettingAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ClaimCoverageProdtProcDef" => {
            // try to parse as ClaimCoverageProdtProcDef
            let parsed: Result<ClaimCoverageProdtProcDef, XmlError> = ClaimCoverageProdtProcDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RevenueManagementSettings" => {
            // try to parse as RevenueManagementSettings
            let parsed: Result<RevenueManagementSettings, XmlError> = RevenueManagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IncludedFeature" => {
            // try to parse as IncludedFeature
            let parsed: Result<IncludedFeature, XmlError> = IncludedFeature::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveAnalyticAssetCollectionItem" => {
            // try to parse as WaveAnalyticAssetCollectionItem
            let parsed: Result<WaveAnalyticAssetCollectionItem, XmlError> = WaveAnalyticAssetCollectionItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmailToCaseRoutingAddress" => {
            // try to parse as EmailToCaseRoutingAddress
            let parsed: Result<EmailToCaseRoutingAddress, XmlError> = EmailToCaseRoutingAddress::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowInputParameterTranslation" => {
            // try to parse as FlowInputParameterTranslation
            let parsed: Result<FlowInputParameterTranslation, XmlError> = FlowInputParameterTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIScoringModelDefinition" => {
            // try to parse as AIScoringModelDefinition
            let parsed: Result<AIScoringModelDefinition, XmlError> = AIScoringModelDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SettingUsageDefinition" => {
            // try to parse as SettingUsageDefinition
            let parsed: Result<SettingUsageDefinition, XmlError> = SettingUsageDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExperiencePropertyTypeBundle" => {
            // try to parse as ExperiencePropertyTypeBundle
            let parsed: Result<ExperiencePropertyTypeBundle, XmlError> = ExperiencePropertyTypeBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SalesDealAgentSettings" => {
            // try to parse as SalesDealAgentSettings
            let parsed: Result<SalesDealAgentSettings, XmlError> = SalesDealAgentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfigGroup" => {
            // try to parse as OmniSupervisorConfigGroup
            let parsed: Result<OmniSupervisorConfigGroup, XmlError> = OmniSupervisorConfigGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIPredictionDefinition" => {
            // try to parse as AIPredictionDefinition
            let parsed: Result<AIPredictionDefinition, XmlError> = AIPredictionDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomTab" => {
            // try to parse as CustomTab
            let parsed: Result<CustomTab, XmlError> = CustomTab::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceCloudNotificationOrchestratorSettings" => {
            // try to parse as ServiceCloudNotificationOrchestratorSettings
            let parsed: Result<ServiceCloudNotificationOrchestratorSettings, XmlError> = ServiceCloudNotificationOrchestratorSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityThemeDefinition" => {
            // try to parse as CommunityThemeDefinition
            let parsed: Result<CommunityThemeDefinition, XmlError> = CommunityThemeDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryModelField" => {
            // try to parse as DiscoveryModelField
            let parsed: Result<DiscoveryModelField, XmlError> = DiscoveryModelField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceChannelStatus" => {
            // try to parse as ServiceChannelStatus
            let parsed: Result<ServiceChannelStatus, XmlError> = ServiceChannelStatus::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventLogObject" => {
            // try to parse as EventLogObject
            let parsed: Result<EventLogObject, XmlError> = EventLogObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetLicenseDefinitionCustomPermission" => {
            // try to parse as PermissionSetLicenseDefinitionCustomPermission
            let parsed: Result<PermissionSetLicenseDefinitionCustomPermission, XmlError> = PermissionSetLicenseDefinitionCustomPermission::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CleanRule" => {
            // try to parse as CleanRule
            let parsed: Result<CleanRule, XmlError> = CleanRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniDataTransform" => {
            // try to parse as OmniDataTransform
            let parsed: Result<OmniDataTransform, XmlError> = OmniDataTransform::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTaskMilestone" => {
            // try to parse as EnablementProgramTaskMilestone
            let parsed: Result<EnablementProgramTaskMilestone, XmlError> = EnablementProgramTaskMilestone::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportGrouping" => {
            // try to parse as ReportGrouping
            let parsed: Result<ReportGrouping, XmlError> = ReportGrouping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DigitalExperienceModule" => {
            // try to parse as DigitalExperienceModule
            let parsed: Result<DigitalExperienceModule, XmlError> = DigitalExperienceModule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SustnUomConversion" => {
            // try to parse as SustnUomConversion
            let parsed: Result<SustnUomConversion, XmlError> = SustnUomConversion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsDatasetDefinition" => {
            // try to parse as AnalyticsDatasetDefinition
            let parsed: Result<AnalyticsDatasetDefinition, XmlError> = AnalyticsDatasetDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowBaseElement" => {
            // try to parse as FlowBaseElement
            let parsed: Result<FlowBaseElement, XmlError> = FlowBaseElement::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SandboxSettings" => {
            // try to parse as SandboxSettings
            let parsed: Result<SandboxSettings, XmlError> = SandboxSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EditionDefinition" => {
            // try to parse as EditionDefinition
            let parsed: Result<EditionDefinition, XmlError> = EditionDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StageDefinition" => {
            // try to parse as StageDefinition
            let parsed: Result<StageDefinition, XmlError> = StageDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingAutoResponse" => {
            // try to parse as MessagingAutoResponse
            let parsed: Result<MessagingAutoResponse, XmlError> = MessagingAutoResponse::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EvfSettings" => {
            // try to parse as EvfSettings
            let parsed: Result<EvfSettings, XmlError> = EvfSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveAgentConfig" => {
            // try to parse as LiveAgentConfig
            let parsed: Result<LiveAgentConfig, XmlError> = LiveAgentConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTest" => {
            // try to parse as FlowTest
            let parsed: Result<FlowTest, XmlError> = FlowTest::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountPlanObjMeasCalcDef" => {
            // try to parse as AccountPlanObjMeasCalcDef
            let parsed: Result<AccountPlanObjMeasCalcDef, XmlError> = AccountPlanObjMeasCalcDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowScreenFieldStyleProperties" => {
            // try to parse as FlowScreenFieldStyleProperties
            let parsed: Result<FlowScreenFieldStyleProperties, XmlError> = FlowScreenFieldStyleProperties::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LifeSciConfigFieldValue" => {
            // try to parse as LifeSciConfigFieldValue
            let parsed: Result<LifeSciConfigFieldValue, XmlError> = LifeSciConfigFieldValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IdentityVerificationFieldTranslation" => {
            // try to parse as IdentityVerificationFieldTranslation
            let parsed: Result<IdentityVerificationFieldTranslation, XmlError> = IdentityVerificationFieldTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageEventTarget" => {
            // try to parse as FlexiPageEventTarget
            let parsed: Result<FlexiPageEventTarget, XmlError> = FlexiPageEventTarget::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTestFlowVersion" => {
            // try to parse as FlowTestFlowVersion
            let parsed: Result<FlowTestFlowVersion, XmlError> = FlowTestFlowVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobTransformAddedField" => {
            // try to parse as BatchCalcJobTransformAddedField
            let parsed: Result<BatchCalcJobTransformAddedField, XmlError> = BatchCalcJobTransformAddedField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountingFieldMapping" => {
            // try to parse as AccountingFieldMapping
            let parsed: Result<AccountingFieldMapping, XmlError> = AccountingFieldMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataConnectionCred" => {
            // try to parse as MktDataConnectionCred
            let parsed: Result<MktDataConnectionCred, XmlError> = MktDataConnectionCred::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DocumentChecklistSettings" => {
            // try to parse as DocumentChecklistSettings
            let parsed: Result<DocumentChecklistSettings, XmlError> = DocumentChecklistSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiEvaluationTestCase" => {
            // try to parse as AiEvaluationTestCase
            let parsed: Result<AiEvaluationTestCase, XmlError> = AiEvaluationTestCase::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PublicKeyCertificate" => {
            // try to parse as PublicKeyCertificate
            let parsed: Result<PublicKeyCertificate, XmlError> = PublicKeyCertificate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlIntent" => {
            // try to parse as MlIntent
            let parsed: Result<MlIntent, XmlError> = MlIntent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActivationPlatformField" => {
            // try to parse as ActivationPlatformField
            let parsed: Result<ActivationPlatformField, XmlError> = ActivationPlatformField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TmfOutboundNotificationSettings" => {
            // try to parse as TmfOutboundNotificationSettings
            let parsed: Result<TmfOutboundNotificationSettings, XmlError> = TmfOutboundNotificationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomMetadata" => {
            // try to parse as CustomMetadata
            let parsed: Result<CustomMetadata, XmlError> = CustomMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SessionSettings" => {
            // try to parse as SessionSettings
            let parsed: Result<SessionSettings, XmlError> = SessionSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppointmentSchedulingPolicy" => {
            // try to parse as AppointmentSchedulingPolicy
            let parsed: Result<AppointmentSchedulingPolicy, XmlError> = AppointmentSchedulingPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PipelineInspMetricConfigTranslation" => {
            // try to parse as PipelineInspMetricConfigTranslation
            let parsed: Result<PipelineInspMetricConfigTranslation, XmlError> = PipelineInspMetricConfigTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvancedObjectMapping" => {
            // try to parse as AdvancedObjectMapping
            let parsed: Result<AdvancedObjectMapping, XmlError> = AdvancedObjectMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomerDataPlatformSettings" => {
            // try to parse as CustomerDataPlatformSettings
            let parsed: Result<CustomerDataPlatformSettings, XmlError> = CustomerDataPlatformSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramSection" => {
            // try to parse as EnablementProgramSection
            let parsed: Result<EnablementProgramSection, XmlError> = EnablementProgramSection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommandActionIntent" => {
            // try to parse as CommandActionIntent
            let parsed: Result<CommandActionIntent, XmlError> = CommandActionIntent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContentAssetVersion" => {
            // try to parse as ContentAssetVersion
            let parsed: Result<ContentAssetVersion, XmlError> = ContentAssetVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectLinkingSettings" => {
            // try to parse as ObjectLinkingSettings
            let parsed: Result<ObjectLinkingSettings, XmlError> = ObjectLinkingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductAttributeSetItem" => {
            // try to parse as ProductAttributeSetItem
            let parsed: Result<ProductAttributeSetItem, XmlError> = ProductAttributeSetItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Flow" => {
            // try to parse as Flow
            let parsed: Result<Flow, XmlError> = Flow::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageConstant" => {
            // try to parse as ConversationMessageConstant
            let parsed: Result<ConversationMessageConstant, XmlError> = ConversationMessageConstant::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkflowEmailRecipient" => {
            // try to parse as WorkflowEmailRecipient
            let parsed: Result<WorkflowEmailRecipient, XmlError> = WorkflowEmailRecipient::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NotificationTypeSettings" => {
            // try to parse as NotificationTypeSettings
            let parsed: Result<NotificationTypeSettings, XmlError> = NotificationTypeSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ManagedTopic" => {
            // try to parse as ManagedTopic
            let parsed: Result<ManagedTopic, XmlError> = ManagedTopic::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchCriteriaConfiguration" => {
            // try to parse as SearchCriteriaConfiguration
            let parsed: Result<SearchCriteriaConfiguration, XmlError> = SearchCriteriaConfiguration::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MatchingRuleItem" => {
            // try to parse as MatchingRuleItem
            let parsed: Result<MatchingRuleItem, XmlError> = MatchingRuleItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CareProviderAfflRoleConfig" => {
            // try to parse as CareProviderAfflRoleConfig
            let parsed: Result<CareProviderAfflRoleConfig, XmlError> = CareProviderAfflRoleConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetElementParameter" => {
            // try to parse as ExpressionSetElementParameter
            let parsed: Result<ExpressionSetElementParameter, XmlError> = ExpressionSetElementParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TrialOrgSettings" => {
            // try to parse as TrialOrgSettings
            let parsed: Result<TrialOrgSettings, XmlError> = TrialOrgSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnlytDshbrdWdgtDynamicTkn" => {
            // try to parse as AnlytDshbrdWdgtDynamicTkn
            let parsed: Result<AnlytDshbrdWdgtDynamicTkn, XmlError> = AnlytDshbrdWdgtDynamicTkn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ValueTranslation" => {
            // try to parse as ValueTranslation
            let parsed: Result<ValueTranslation, XmlError> = ValueTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryGoalOutcome" => {
            // try to parse as DiscoveryGoalOutcome
            let parsed: Result<DiscoveryGoalOutcome, XmlError> = DiscoveryGoalOutcome::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetFlowAccess" => {
            // try to parse as PermissionSetFlowAccess
            let parsed: Result<PermissionSetFlowAccess, XmlError> = PermissionSetFlowAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingObjectListSelectedSettings" => {
            // try to parse as ForecastingObjectListSelectedSettings
            let parsed: Result<ForecastingObjectListSelectedSettings, XmlError> = ForecastingObjectListSelectedSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportTypeColumn" => {
            // try to parse as ReportTypeColumn
            let parsed: Result<ReportTypeColumn, XmlError> = ReportTypeColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkflowRule" => {
            // try to parse as WorkflowRule
            let parsed: Result<WorkflowRule, XmlError> = WorkflowRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PicklistValueTranslation" => {
            // try to parse as PicklistValueTranslation
            let parsed: Result<PicklistValueTranslation, XmlError> = PicklistValueTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RetrieveResult" => {
            // try to parse as RetrieveResult
            let parsed: Result<RetrieveResult, XmlError> = RetrieveResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectSourceTargetMap" => {
            // try to parse as ObjectSourceTargetMap
            let parsed: Result<ObjectSourceTargetMap, XmlError> = ObjectSourceTargetMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetObjectAlias" => {
            // try to parse as ExpressionSetObjectAlias
            let parsed: Result<ExpressionSetObjectAlias, XmlError> = ExpressionSetObjectAlias::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmployeeUserSettings" => {
            // try to parse as EmployeeUserSettings
            let parsed: Result<EmployeeUserSettings, XmlError> = EmployeeUserSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdjustmentsSettings" => {
            // try to parse as AdjustmentsSettings
            let parsed: Result<AdjustmentsSettings, XmlError> = AdjustmentsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsParamWidgetDef" => {
            // try to parse as AnalyticsParamWidgetDef
            let parsed: Result<AnalyticsParamWidgetDef, XmlError> = AnalyticsParamWidgetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2RuleItem" => {
            // try to parse as Territory2RuleItem
            let parsed: Result<Territory2RuleItem, XmlError> = Territory2RuleItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotVariableOperand" => {
            // try to parse as BotVariableOperand
            let parsed: Result<BotVariableOperand, XmlError> = BotVariableOperand::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalDataTranField" => {
            // try to parse as ExternalDataTranField
            let parsed: Result<ExternalDataTranField, XmlError> = ExternalDataTranField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeWorkOrderLineItemFieldsSettings" => {
            // try to parse as KnowledgeWorkOrderLineItemFieldsSettings
            let parsed: Result<KnowledgeWorkOrderLineItemFieldsSettings, XmlError> = KnowledgeWorkOrderLineItemFieldsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StrategyAction" => {
            // try to parse as StrategyAction
            let parsed: Result<StrategyAction, XmlError> = StrategyAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserInterfaceSettings" => {
            // try to parse as UserInterfaceSettings
            let parsed: Result<UserInterfaceSettings, XmlError> = UserInterfaceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardFilterColumn" => {
            // try to parse as DashboardFilterColumn
            let parsed: Result<DashboardFilterColumn, XmlError> = DashboardFilterColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowSchedule" => {
            // try to parse as FlowSchedule
            let parsed: Result<FlowSchedule, XmlError> = FlowSchedule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CallCtrAgentFavTrfrDest" => {
            // try to parse as CallCtrAgentFavTrfrDest
            let parsed: Result<CallCtrAgentFavTrfrDest, XmlError> = CallCtrAgentFavTrfrDest::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportAggregateReference" => {
            // try to parse as ReportAggregateReference
            let parsed: Result<ReportAggregateReference, XmlError> = ReportAggregateReference::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PolicyRuleDefinitionSet" => {
            // try to parse as PolicyRuleDefinitionSet
            let parsed: Result<PolicyRuleDefinitionSet, XmlError> = PolicyRuleDefinitionSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationDefinitionGoal" => {
            // try to parse as ConversationDefinitionGoal
            let parsed: Result<ConversationDefinitionGoal, XmlError> = ConversationDefinitionGoal::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppointmentAssignmentPolicy" => {
            // try to parse as AppointmentAssignmentPolicy
            let parsed: Result<AppointmentAssignmentPolicy, XmlError> = AppointmentAssignmentPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageLayoutItem" => {
            // try to parse as ConversationMessageLayoutItem
            let parsed: Result<ConversationMessageLayoutItem, XmlError> = ConversationMessageLayoutItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceAIRecommendationsSettings" => {
            // try to parse as ServiceAIRecommendationsSettings
            let parsed: Result<ServiceAIRecommendationsSettings, XmlError> = ServiceAIRecommendationsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommsUpsellSettings" => {
            // try to parse as CommsUpsellSettings
            let parsed: Result<CommsUpsellSettings, XmlError> = CommsUpsellSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlannerRuleExprCondition" => {
            // try to parse as GenAiPlannerRuleExprCondition
            let parsed: Result<GenAiPlannerRuleExprCondition, XmlError> = GenAiPlannerRuleExprCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CareProviderSearchConfig" => {
            // try to parse as CareProviderSearchConfig
            let parsed: Result<CareProviderSearchConfig, XmlError> = CareProviderSearchConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextNodeMapping" => {
            // try to parse as ContextNodeMapping
            let parsed: Result<ContextNodeMapping, XmlError> = ContextNodeMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Layout" => {
            // try to parse as Layout
            let parsed: Result<Layout, XmlError> = Layout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FTestAccessSettings" => {
            // try to parse as FTestAccessSettings
            let parsed: Result<FTestAccessSettings, XmlError> = FTestAccessSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotVersion" => {
            // try to parse as BotVersion
            let parsed: Result<BotVersion, XmlError> = BotVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OrchestrationContext" => {
            // try to parse as OrchestrationContext
            let parsed: Result<OrchestrationContext, XmlError> = OrchestrationContext::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlParameterDefinition" => {
            // try to parse as MlParameterDefinition
            let parsed: Result<MlParameterDefinition, XmlError> = MlParameterDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlModelOutput" => {
            // try to parse as MlModelOutput
            let parsed: Result<MlModelOutput, XmlError> = MlModelOutput::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementMeasureFilterDefinition" => {
            // try to parse as EnablementMeasureFilterDefinition
            let parsed: Result<EnablementMeasureFilterDefinition, XmlError> = EnablementMeasureFilterDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RoleOrTerritory" => {
            // try to parse as RoleOrTerritory
            let parsed: Result<RoleOrTerritory, XmlError> = RoleOrTerritory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BtchCalcJobFrcstAggrFld" => {
            // try to parse as BtchCalcJobFrcstAggrFld
            let parsed: Result<BtchCalcJobFrcstAggrFld, XmlError> = BtchCalcJobFrcstAggrFld::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SkillAssignments" => {
            // try to parse as SkillAssignments
            let parsed: Result<SkillAssignments, XmlError> = SkillAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountingSettings" => {
            // try to parse as AccountingSettings
            let parsed: Result<AccountingSettings, XmlError> = AccountingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FileUploadAndDownloadSecuritySettings" => {
            // try to parse as FileUploadAndDownloadSecuritySettings
            let parsed: Result<FileUploadAndDownloadSecuritySettings, XmlError> = FileUploadAndDownloadSecuritySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PathAssistant" => {
            // try to parse as PathAssistant
            let parsed: Result<PathAssistant, XmlError> = PathAssistant::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedRecordAccessFltr" => {
            // try to parse as RelatedRecordAccessFltr
            let parsed: Result<RelatedRecordAccessFltr, XmlError> = RelatedRecordAccessFltr::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CollectionsDashboardSettings" => {
            // try to parse as CollectionsDashboardSettings
            let parsed: Result<CollectionsDashboardSettings, XmlError> = CollectionsDashboardSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ModuleRef" => {
            // try to parse as ModuleRef
            let parsed: Result<ModuleRef, XmlError> = ModuleRef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PriceRuleAction" => {
            // try to parse as PriceRuleAction
            let parsed: Result<PriceRuleAction, XmlError> = PriceRuleAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Module" => {
            // try to parse as Module
            let parsed: Result<Module, XmlError> = Module::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportTypeSectionTranslation" => {
            // try to parse as ReportTypeSectionTranslation
            let parsed: Result<ReportTypeSectionTranslation, XmlError> = ReportTypeSectionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PricingActionParameters" => {
            // try to parse as PricingActionParameters
            let parsed: Result<PricingActionParameters, XmlError> = PricingActionParameters::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PriceRuleExecutionStage" => {
            // try to parse as PriceRuleExecutionStage
            let parsed: Result<PriceRuleExecutionStage, XmlError> = PriceRuleExecutionStage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InvocableActionExtensionTargetAttribute" => {
            // try to parse as InvocableActionExtensionTargetAttribute
            let parsed: Result<InvocableActionExtensionTargetAttribute, XmlError> = InvocableActionExtensionTargetAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DocumentType" => {
            // try to parse as DocumentType
            let parsed: Result<DocumentType, XmlError> = DocumentType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReputationBranding" => {
            // try to parse as ReputationBranding
            let parsed: Result<ReputationBranding, XmlError> = ReputationBranding::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorIngestApi" => {
            // try to parse as DataConnectorIngestApi
            let parsed: Result<DataConnectorIngestApi, XmlError> = DataConnectorIngestApi::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PersonAccountOwnerPowerUser" => {
            // try to parse as PersonAccountOwnerPowerUser
            let parsed: Result<PersonAccountOwnerPowerUser, XmlError> = PersonAccountOwnerPowerUser::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcess" => {
            // try to parse as LoyaltyProgramProcess
            let parsed: Result<LoyaltyProgramProcess, XmlError> = LoyaltyProgramProcess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmployeeDataSyncField" => {
            // try to parse as EmployeeDataSyncField
            let parsed: Result<EmployeeDataSyncField, XmlError> = EmployeeDataSyncField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Container" => {
            // try to parse as Container
            let parsed: Result<Container, XmlError> = Container::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceForm" => {
            // try to parse as EmbeddedServiceForm
            let parsed: Result<EmbeddedServiceForm, XmlError> = EmbeddedServiceForm::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RemoteSiteSetting" => {
            // try to parse as RemoteSiteSetting
            let parsed: Result<RemoteSiteSetting, XmlError> = RemoteSiteSetting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Folder" => {
            // try to parse as Folder
            let parsed: Result<Folder, XmlError> = Folder::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Application" => {
            // try to parse as Application
            let parsed: Result<Application, XmlError> = Application::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportBucketFieldValue" => {
            // try to parse as ReportBucketFieldValue
            let parsed: Result<ReportBucketFieldValue, XmlError> = ReportBucketFieldValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiEvalCopilotTestCaseCntxtVar" => {
            // try to parse as AiEvalCopilotTestCaseCntxtVar
            let parsed: Result<AiEvalCopilotTestCaseCntxtVar, XmlError> = AiEvalCopilotTestCaseCntxtVar::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceMessagingChannel" => {
            // try to parse as EmbeddedServiceMessagingChannel
            let parsed: Result<EmbeddedServiceMessagingChannel, XmlError> = EmbeddedServiceMessagingChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsDashboard" => {
            // try to parse as AnalyticsDashboard
            let parsed: Result<AnalyticsDashboard, XmlError> = AnalyticsDashboard::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIPredictionTarget" => {
            // try to parse as AIPredictionTarget
            let parsed: Result<AIPredictionTarget, XmlError> = AIPredictionTarget::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowSettings" => {
            // try to parse as FlowSettings
            let parsed: Result<FlowSettings, XmlError> = FlowSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataConnectionSrcParam" => {
            // try to parse as MktDataConnectionSrcParam
            let parsed: Result<MktDataConnectionSrcParam, XmlError> = MktDataConnectionSrcParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MacroSettings" => {
            // try to parse as MacroSettings
            let parsed: Result<MacroSettings, XmlError> = MacroSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SubUnnamedChildFTestMd1" => {
            // try to parse as SubUnnamedChildFTestMd1
            let parsed: Result<SubUnnamedChildFTestMd1, XmlError> = SubUnnamedChildFTestMd1::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectHierarchyRelationship" => {
            // try to parse as ObjectHierarchyRelationship
            let parsed: Result<ObjectHierarchyRelationship, XmlError> = ObjectHierarchyRelationship::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingChannelUsage" => {
            // try to parse as MessagingChannelUsage
            let parsed: Result<MessagingChannelUsage, XmlError> = MessagingChannelUsage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MobSecurityCertPinConfig" => {
            // try to parse as MobSecurityCertPinConfig
            let parsed: Result<MobSecurityCertPinConfig, XmlError> = MobSecurityCertPinConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalCredentialParameter" => {
            // try to parse as ExternalCredentialParameter
            let parsed: Result<ExternalCredentialParameter, XmlError> = ExternalCredentialParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowInputValidationRuleTranslation" => {
            // try to parse as FlowInputValidationRuleTranslation
            let parsed: Result<FlowInputValidationRuleTranslation, XmlError> = FlowInputValidationRuleTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ValueSet" => {
            // try to parse as ValueSet
            let parsed: Result<ValueSet, XmlError> = ValueSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeedItemSettings" => {
            // try to parse as FeedItemSettings
            let parsed: Result<FeedItemSettings, XmlError> = FeedItemSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConvMsgExternalTemplateVersion" => {
            // try to parse as ConvMsgExternalTemplateVersion
            let parsed: Result<ConvMsgExternalTemplateVersion, XmlError> = ConvMsgExternalTemplateVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingGroupItem" => {
            // try to parse as ForecastingGroupItem
            let parsed: Result<ForecastingGroupItem, XmlError> = ForecastingGroupItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SalesAgreementSettings" => {
            // try to parse as SalesAgreementSettings
            let parsed: Result<SalesAgreementSettings, XmlError> = SalesAgreementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataModelTaxonomy" => {
            // try to parse as DataModelTaxonomy
            let parsed: Result<DataModelTaxonomy, XmlError> = DataModelTaxonomy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChoiceList" => {
            // try to parse as ChoiceList
            let parsed: Result<ChoiceList, XmlError> = ChoiceList::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FileProperties" => {
            // try to parse as FileProperties
            let parsed: Result<FileProperties, XmlError> = FileProperties::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RetrievalSummaryDefField" => {
            // try to parse as RetrievalSummaryDefField
            let parsed: Result<RetrievalSummaryDefField, XmlError> = RetrievalSummaryDefField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTaskMilestoneMeasure" => {
            // try to parse as EnablementProgramTaskMilestoneMeasure
            let parsed: Result<EnablementProgramTaskMilestoneMeasure, XmlError> = EnablementProgramTaskMilestoneMeasure::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Site" => {
            // try to parse as Site
            let parsed: Result<Site, XmlError> = Site::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileActionOverride" => {
            // try to parse as ProfileActionOverride
            let parsed: Result<ProfileActionOverride, XmlError> = ProfileActionOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsDashboardPage" => {
            // try to parse as AnalyticsDashboardPage
            let parsed: Result<AnalyticsDashboardPage, XmlError> = AnalyticsDashboardPage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EntitlementProcessMilestoneItem" => {
            // try to parse as EntitlementProcessMilestoneItem
            let parsed: Result<EntitlementProcessMilestoneItem, XmlError> = EntitlementProcessMilestoneItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetCustomElement" => {
            // try to parse as ExpressionSetCustomElement
            let parsed: Result<ExpressionSetCustomElement, XmlError> = ExpressionSetCustomElement::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementMeasureDefinition" => {
            // try to parse as EnablementMeasureDefinition
            let parsed: Result<EnablementMeasureDefinition, XmlError> = EnablementMeasureDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageRegion" => {
            // try to parse as FlexiPageRegion
            let parsed: Result<FlexiPageRegion, XmlError> = FlexiPageRegion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BenefitAction" => {
            // try to parse as BenefitAction
            let parsed: Result<BenefitAction, XmlError> = BenefitAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TransactionSecurityNotification" => {
            // try to parse as TransactionSecurityNotification
            let parsed: Result<TransactionSecurityNotification, XmlError> = TransactionSecurityNotification::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomSite" => {
            // try to parse as CustomSite
            let parsed: Result<CustomSite, XmlError> = CustomSite::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestSecondTopLevel" => {
            // try to parse as FtestSecondTopLevel
            let parsed: Result<FtestSecondTopLevel, XmlError> = FtestSecondTopLevel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectivityDevConfigMetadata" => {
            // try to parse as ConnectivityDevConfigMetadata
            let parsed: Result<ConnectivityDevConfigMetadata, XmlError> = ConnectivityDevConfigMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordAggregationJoinCondition" => {
            // try to parse as RecordAggregationJoinCondition
            let parsed: Result<RecordAggregationJoinCondition, XmlError> = RecordAggregationJoinCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2SupportedObject" => {
            // try to parse as Territory2SupportedObject
            let parsed: Result<Territory2SupportedObject, XmlError> = Territory2SupportedObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIPredictionField" => {
            // try to parse as AIPredictionField
            let parsed: Result<AIPredictionField, XmlError> = AIPredictionField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnector" => {
            // try to parse as DataConnector
            let parsed: Result<DataConnector, XmlError> = DataConnector::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionTableParameter" => {
            // try to parse as DecisionTableParameter
            let parsed: Result<DecisionTableParameter, XmlError> = DecisionTableParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAcctForecastDimension" => {
            // try to parse as AdvAcctForecastDimension
            let parsed: Result<AdvAcctForecastDimension, XmlError> = AdvAcctForecastDimension::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EntitlementSettings" => {
            // try to parse as EntitlementSettings
            let parsed: Result<EntitlementSettings, XmlError> = EntitlementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExplainabilityMessageTemplateTokenMapping" => {
            // try to parse as ExplainabilityMessageTemplateTokenMapping
            let parsed: Result<ExplainabilityMessageTemplateTokenMapping, XmlError> = ExplainabilityMessageTemplateTokenMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2Rule" => {
            // try to parse as Territory2Rule
            let parsed: Result<Territory2Rule, XmlError> = Territory2Rule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SiteRedirectMapping" => {
            // try to parse as SiteRedirectMapping
            let parsed: Result<SiteRedirectMapping, XmlError> = SiteRedirectMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramDefinition" => {
            // try to parse as EnablementProgramDefinition
            let parsed: Result<EnablementProgramDefinition, XmlError> = EnablementProgramDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppMobileConfigurablePolicies" => {
            // try to parse as ExtlClntAppMobileConfigurablePolicies
            let parsed: Result<ExtlClntAppMobileConfigurablePolicies, XmlError> = ExtlClntAppMobileConfigurablePolicies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QueueSobject" => {
            // try to parse as QueueSobject
            let parsed: Result<QueueSobject, XmlError> = QueueSobject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotBlockVersion" => {
            // try to parse as BotBlockVersion
            let parsed: Result<BotBlockVersion, XmlError> = BotBlockVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobDatasourceField" => {
            // try to parse as BatchCalcJobDatasourceField
            let parsed: Result<BatchCalcJobDatasourceField, XmlError> = BatchCalcJobDatasourceField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContractSettings" => {
            // try to parse as ContractSettings
            let parsed: Result<ContractSettings, XmlError> = ContractSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MapsAndLocationSettings" => {
            // try to parse as MapsAndLocationSettings
            let parsed: Result<MapsAndLocationSettings, XmlError> = MapsAndLocationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductAttributeSet" => {
            // try to parse as ProductAttributeSet
            let parsed: Result<ProductAttributeSet, XmlError> = ProductAttributeSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FTestFieldMappingMd" => {
            // try to parse as FTestFieldMappingMd
            let parsed: Result<FTestFieldMappingMd, XmlError> = FTestFieldMappingMd::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CodeCoverageResult" => {
            // try to parse as CodeCoverageResult
            let parsed: Result<CodeCoverageResult, XmlError> = CodeCoverageResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordActionRecommendation" => {
            // try to parse as RecordActionRecommendation
            let parsed: Result<RecordActionRecommendation, XmlError> = RecordActionRecommendation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowCategoryItems" => {
            // try to parse as FlowCategoryItems
            let parsed: Result<FlowCategoryItems, XmlError> = FlowCategoryItems::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesGamificationSettings" => {
            // try to parse as IndustriesGamificationSettings
            let parsed: Result<IndustriesGamificationSettings, XmlError> = IndustriesGamificationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceAISetupDefinition" => {
            // try to parse as ServiceAISetupDefinition
            let parsed: Result<ServiceAISetupDefinition, XmlError> = ServiceAISetupDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeLanguageSettings" => {
            // try to parse as KnowledgeLanguageSettings
            let parsed: Result<KnowledgeLanguageSettings, XmlError> = KnowledgeLanguageSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActivitiesSettings" => {
            // try to parse as ActivitiesSettings
            let parsed: Result<ActivitiesSettings, XmlError> = ActivitiesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiPluginUtteranceDef" => {
            // try to parse as AiPluginUtteranceDef
            let parsed: Result<AiPluginUtteranceDef, XmlError> = AiPluginUtteranceDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetAggregation" => {
            // try to parse as ExpressionSetAggregation
            let parsed: Result<ExpressionSetAggregation, XmlError> = ExpressionSetAggregation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingChannelParameterValueMapping" => {
            // try to parse as MessagingChannelParameterValueMapping
            let parsed: Result<MessagingChannelParameterValueMapping, XmlError> = MessagingChannelParameterValueMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalClientApplication" => {
            // try to parse as ExternalClientApplication
            let parsed: Result<ExternalClientApplication, XmlError> = ExternalClientApplication::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotQuickReplyOptionTranslation" => {
            // try to parse as BotQuickReplyOptionTranslation
            let parsed: Result<BotQuickReplyOptionTranslation, XmlError> = BotQuickReplyOptionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPromptTemplateActv" => {
            // try to parse as GenAiPromptTemplateActv
            let parsed: Result<GenAiPromptTemplateActv, XmlError> = GenAiPromptTemplateActv::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalPageField" => {
            // try to parse as ApprovalPageField
            let parsed: Result<ApprovalPageField, XmlError> = ApprovalPageField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountSettings" => {
            // try to parse as AccountSettings
            let parsed: Result<AccountSettings, XmlError> = AccountSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniExtTrackingEventDef" => {
            // try to parse as OmniExtTrackingEventDef
            let parsed: Result<OmniExtTrackingEventDef, XmlError> = OmniExtTrackingEventDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SchedulingObjectiveParameter" => {
            // try to parse as SchedulingObjectiveParameter
            let parsed: Result<SchedulingObjectiveParameter, XmlError> = SchedulingObjectiveParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectUsage" => {
            // try to parse as ObjectUsage
            let parsed: Result<ObjectUsage, XmlError> = ObjectUsage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeWorkOrderFieldsSettings" => {
            // try to parse as KnowledgeWorkOrderFieldsSettings
            let parsed: Result<KnowledgeWorkOrderFieldsSettings, XmlError> = KnowledgeWorkOrderFieldsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportColorRange" => {
            // try to parse as ReportColorRange
            let parsed: Result<ReportColorRange, XmlError> = ReportColorRange::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Report" => {
            // try to parse as Report
            let parsed: Result<Report, XmlError> = Report::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedRecordAccessDef" => {
            // try to parse as RelatedRecordAccessDef
            let parsed: Result<RelatedRecordAccessDef, XmlError> = RelatedRecordAccessDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSet" => {
            // try to parse as PermissionSet
            let parsed: Result<PermissionSet, XmlError> = PermissionSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StnryAssetEnvSrcCnfg" => {
            // try to parse as StnryAssetEnvSrcCnfg
            let parsed: Result<StnryAssetEnvSrcCnfg, XmlError> = StnryAssetEnvSrcCnfg::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetServicePresenceStatusAccess" => {
            // try to parse as PermissionSetServicePresenceStatusAccess
            let parsed: Result<PermissionSetServicePresenceStatusAccess, XmlError> = PermissionSetServicePresenceStatusAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlannerFunctionDef" => {
            // try to parse as GenAiPlannerFunctionDef
            let parsed: Result<GenAiPlannerFunctionDef, XmlError> = GenAiPlannerFunctionDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportBucketFieldSourceValue" => {
            // try to parse as ReportBucketFieldSourceValue
            let parsed: Result<ReportBucketFieldSourceValue, XmlError> = ReportBucketFieldSourceValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTestCondition" => {
            // try to parse as FlowTestCondition
            let parsed: Result<FlowTestCondition, XmlError> = FlowTestCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesEinsteinFeatureSettings" => {
            // try to parse as IndustriesEinsteinFeatureSettings
            let parsed: Result<IndustriesEinsteinFeatureSettings, XmlError> = IndustriesEinsteinFeatureSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniProcessElement" => {
            // try to parse as OmniProcessElement
            let parsed: Result<OmniProcessElement, XmlError> = OmniProcessElement::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionableEventOrchDef" => {
            // try to parse as ActionableEventOrchDef
            let parsed: Result<ActionableEventOrchDef, XmlError> = ActionableEventOrchDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationDefinitionDetail" => {
            // try to parse as RecommendationDefinitionDetail
            let parsed: Result<RecommendationDefinitionDetail, XmlError> = RecommendationDefinitionDetail::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EncryptionKeySettings" => {
            // try to parse as EncryptionKeySettings
            let parsed: Result<EncryptionKeySettings, XmlError> = EncryptionKeySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextMappingConfig" => {
            // try to parse as ContextMappingConfig
            let parsed: Result<ContextMappingConfig, XmlError> = ContextMappingConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardTableColumn" => {
            // try to parse as DashboardTableColumn
            let parsed: Result<DashboardTableColumn, XmlError> = DashboardTableColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChannelLayout" => {
            // try to parse as ChannelLayout
            let parsed: Result<ChannelLayout, XmlError> = ChannelLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MetadataWithContent" => {
            // try to parse as MetadataWithContent
            let parsed: Result<MetadataWithContent, XmlError> = MetadataWithContent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PromptVersion" => {
            // try to parse as PromptVersion
            let parsed: Result<PromptVersion, XmlError> = PromptVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveDataset" => {
            // try to parse as WaveDataset
            let parsed: Result<WaveDataset, XmlError> = WaveDataset::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SummaryLayoutItem" => {
            // try to parse as SummaryLayoutItem
            let parsed: Result<SummaryLayoutItem, XmlError> = SummaryLayoutItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchCustomization" => {
            // try to parse as SearchCustomization
            let parsed: Result<SearchCustomization, XmlError> = SearchCustomization::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportFilter" => {
            // try to parse as ReportFilter
            let parsed: Result<ReportFilter, XmlError> = ReportFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppAndroidPushConfig" => {
            // try to parse as ExtlClntAppAndroidPushConfig
            let parsed: Result<ExtlClntAppAndroidPushConfig, XmlError> = ExtlClntAppAndroidPushConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PartyDataModelSettings" => {
            // try to parse as PartyDataModelSettings
            let parsed: Result<PartyDataModelSettings, XmlError> = PartyDataModelSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PolicyRuleValueSet" => {
            // try to parse as PolicyRuleValueSet
            let parsed: Result<PolicyRuleValueSet, XmlError> = PolicyRuleValueSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppSamlConfigurablePolicies" => {
            // try to parse as ExtlClntAppSamlConfigurablePolicies
            let parsed: Result<ExtlClntAppSamlConfigurablePolicies, XmlError> = ExtlClntAppSamlConfigurablePolicies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalServiceRegistration" => {
            // try to parse as ExternalServiceRegistration
            let parsed: Result<ExternalServiceRegistration, XmlError> = ExternalServiceRegistration::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataSourceTenant" => {
            // try to parse as DataSourceTenant
            let parsed: Result<DataSourceTenant, XmlError> = DataSourceTenant::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationLoadCondition" => {
            // try to parse as RecommendationLoadCondition
            let parsed: Result<RecommendationLoadCondition, XmlError> = RecommendationLoadCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Portal" => {
            // try to parse as Portal
            let parsed: Result<Portal, XmlError> = Portal::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppSampleSettings" => {
            // try to parse as ExtlClntAppSampleSettings
            let parsed: Result<ExtlClntAppSampleSettings, XmlError> = ExtlClntAppSampleSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppMobileDetailConfig" => {
            // try to parse as ConnectedAppMobileDetailConfig
            let parsed: Result<ConnectedAppMobileDetailConfig, XmlError> = ConnectedAppMobileDetailConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageParameter" => {
            // try to parse as ConversationMessageParameter
            let parsed: Result<ConversationMessageParameter, XmlError> = ConversationMessageParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EinsteinAISettings" => {
            // try to parse as EinsteinAISettings
            let parsed: Result<EinsteinAISettings, XmlError> = EinsteinAISettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssessmentQuestionVersionChoice" => {
            // try to parse as AssessmentQuestionVersionChoice
            let parsed: Result<AssessmentQuestionVersionChoice, XmlError> = AssessmentQuestionVersionChoice::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MarketAudienceDefinition" => {
            // try to parse as MarketAudienceDefinition
            let parsed: Result<MarketAudienceDefinition, XmlError> = MarketAudienceDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventParameterMap" => {
            // try to parse as EventParameterMap
            let parsed: Result<EventParameterMap, XmlError> = EventParameterMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomTabTranslation" => {
            // try to parse as CustomTabTranslation
            let parsed: Result<CustomTabTranslation, XmlError> = CustomTabTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OrchestrationContextDataset" => {
            // try to parse as OrchestrationContextDataset
            let parsed: Result<OrchestrationContextDataset, XmlError> = OrchestrationContextDataset::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChartSummary" => {
            // try to parse as ChartSummary
            let parsed: Result<ChartSummary, XmlError> = ChartSummary::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DuplicateRuleMatchRule" => {
            // try to parse as DuplicateRuleMatchRule
            let parsed: Result<DuplicateRuleMatchRule, XmlError> = DuplicateRuleMatchRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetRecordTypeVisibility" => {
            // try to parse as PermissionSetRecordTypeVisibility
            let parsed: Result<PermissionSetRecordTypeVisibility, XmlError> = PermissionSetRecordTypeVisibility::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobAggregateField" => {
            // try to parse as BatchCalcJobAggregateField
            let parsed: Result<BatchCalcJobAggregateField, XmlError> = BatchCalcJobAggregateField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogItemAttrDetail" => {
            // try to parse as SvcCatalogItemAttrDetail
            let parsed: Result<SvcCatalogItemAttrDetail, XmlError> = SvcCatalogItemAttrDetail::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReferralMarketingConfig" => {
            // try to parse as ReferralMarketingConfig
            let parsed: Result<ReferralMarketingConfig, XmlError> = ReferralMarketingConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserAccessPolicyAction" => {
            // try to parse as UserAccessPolicyAction
            let parsed: Result<UserAccessPolicyAction, XmlError> = UserAccessPolicyAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomMetadataValue" => {
            // try to parse as CustomMetadataValue
            let parsed: Result<CustomMetadataValue, XmlError> = CustomMetadataValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsMetricWidgetDef" => {
            // try to parse as AnalyticsMetricWidgetDef
            let parsed: Result<AnalyticsMetricWidgetDef, XmlError> = AnalyticsMetricWidgetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAccountForecastFormula" => {
            // try to parse as AdvAccountForecastFormula
            let parsed: Result<AdvAccountForecastFormula, XmlError> = AdvAccountForecastFormula::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Translations" => {
            // try to parse as Translations
            let parsed: Result<Translations, XmlError> = Translations::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DigitalExperienceModuleCollection" => {
            // try to parse as DigitalExperienceModuleCollection
            let parsed: Result<DigitalExperienceModuleCollection, XmlError> = DigitalExperienceModuleCollection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPage" => {
            // try to parse as FlexiPage
            let parsed: Result<FlexiPage, XmlError> = FlexiPage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DeployDetails" => {
            // try to parse as DeployDetails
            let parsed: Result<DeployDetails, XmlError> = DeployDetails::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OpportunityScoreSettings" => {
            // try to parse as OpportunityScoreSettings
            let parsed: Result<OpportunityScoreSettings, XmlError> = OpportunityScoreSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectRelationship" => {
            // try to parse as ObjectRelationship
            let parsed: Result<ObjectRelationship, XmlError> = ObjectRelationship::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StaticDynamicValMapping" => {
            // try to parse as StaticDynamicValMapping
            let parsed: Result<StaticDynamicValMapping, XmlError> = StaticDynamicValMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IpRange" => {
            // try to parse as IpRange
            let parsed: Result<IpRange, XmlError> = IpRange::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Network" => {
            // try to parse as Network
            let parsed: Result<Network, XmlError> = Network::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServicePresenceStatus" => {
            // try to parse as ServicePresenceStatus
            let parsed: Result<ServicePresenceStatus, XmlError> = ServicePresenceStatus::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsTextWidgetDef" => {
            // try to parse as AnalyticsTextWidgetDef
            let parsed: Result<AnalyticsTextWidgetDef, XmlError> = AnalyticsTextWidgetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsContainerWidgetDef" => {
            // try to parse as AnalyticsContainerWidgetDef
            let parsed: Result<AnalyticsContainerWidgetDef, XmlError> = AnalyticsContainerWidgetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VirtualVisitConfig" => {
            // try to parse as VirtualVisitConfig
            let parsed: Result<VirtualVisitConfig, XmlError> = VirtualVisitConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageLayout" => {
            // try to parse as ConversationMessageLayout
            let parsed: Result<ConversationMessageLayout, XmlError> = ConversationMessageLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MLPredictionDefinition" => {
            // try to parse as MLPredictionDefinition
            let parsed: Result<MLPredictionDefinition, XmlError> = MLPredictionDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PaymentsSettings" => {
            // try to parse as PaymentsSettings
            let parsed: Result<PaymentsSettings, XmlError> = PaymentsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlModelSchema" => {
            // try to parse as MlModelSchema
            let parsed: Result<MlModelSchema, XmlError> = MlModelSchema::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AffinityScoreDefinition" => {
            // try to parse as AffinityScoreDefinition
            let parsed: Result<AffinityScoreDefinition, XmlError> = AffinityScoreDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserAccessPolicy" => {
            // try to parse as UserAccessPolicy
            let parsed: Result<UserAccessPolicy, XmlError> = UserAccessPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiEvaluationAgentTestCaseInput" => {
            // try to parse as AiEvaluationAgentTestCaseInput
            let parsed: Result<AiEvaluationAgentTestCaseInput, XmlError> = AiEvaluationAgentTestCaseInput::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeCommunitiesSettings" => {
            // try to parse as KnowledgeCommunitiesSettings
            let parsed: Result<KnowledgeCommunitiesSettings, XmlError> = KnowledgeCommunitiesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppCanvasConfig" => {
            // try to parse as ConnectedAppCanvasConfig
            let parsed: Result<ConnectedAppCanvasConfig, XmlError> = ConnectedAppCanvasConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GlobalPicklist" => {
            // try to parse as GlobalPicklist
            let parsed: Result<GlobalPicklist, XmlError> = GlobalPicklist::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityTemplateDefinition" => {
            // try to parse as CommunityTemplateDefinition
            let parsed: Result<CommunityTemplateDefinition, XmlError> = CommunityTemplateDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdOrganization" => {
            // try to parse as WaveXmdOrganization
            let parsed: Result<WaveXmdOrganization, XmlError> = WaveXmdOrganization::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NetworkAuthApiSettings" => {
            // try to parse as NetworkAuthApiSettings
            let parsed: Result<NetworkAuthApiSettings, XmlError> = NetworkAuthApiSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesUsageSettings" => {
            // try to parse as IndustriesUsageSettings
            let parsed: Result<IndustriesUsageSettings, XmlError> = IndustriesUsageSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportFormattingRule" => {
            // try to parse as ReportFormattingRule
            let parsed: Result<ReportFormattingRule, XmlError> = ReportFormattingRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ListViewFilter" => {
            // try to parse as ListViewFilter
            let parsed: Result<ListViewFilter, XmlError> = ListViewFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppSettings" => {
            // try to parse as AppSettings
            let parsed: Result<AppSettings, XmlError> = AppSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssessmentQuestionSet" => {
            // try to parse as AssessmentQuestionSet
            let parsed: Result<AssessmentQuestionSet, XmlError> = AssessmentQuestionSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApexEmailNotifications" => {
            // try to parse as ApexEmailNotifications
            let parsed: Result<ApexEmailNotifications, XmlError> = ApexEmailNotifications::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OrderManagementSettings" => {
            // try to parse as OrderManagementSettings
            let parsed: Result<OrderManagementSettings, XmlError> = OrderManagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DigitalExperienceFolderShares" => {
            // try to parse as DigitalExperienceFolderShares
            let parsed: Result<DigitalExperienceFolderShares, XmlError> = DigitalExperienceFolderShares::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIFeatureExtractor" => {
            // try to parse as AIFeatureExtractor
            let parsed: Result<AIFeatureExtractor, XmlError> = AIFeatureExtractor::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InsPolicyLifecycleConfig" => {
            // try to parse as InsPolicyLifecycleConfig
            let parsed: Result<InsPolicyLifecycleConfig, XmlError> = InsPolicyLifecycleConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssessmentQuestion" => {
            // try to parse as AssessmentQuestion
            let parsed: Result<AssessmentQuestion, XmlError> = AssessmentQuestion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryFilterValue" => {
            // try to parse as DiscoveryFilterValue
            let parsed: Result<DiscoveryFilterValue, XmlError> = DiscoveryFilterValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTranslation" => {
            // try to parse as FlowTranslation
            let parsed: Result<FlowTranslation, XmlError> = FlowTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcessCondition" => {
            // try to parse as LoyaltyProgramProcessCondition
            let parsed: Result<LoyaltyProgramProcessCondition, XmlError> = LoyaltyProgramProcessCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextDefinition" => {
            // try to parse as ContextDefinition
            let parsed: Result<ContextDefinition, XmlError> = ContextDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CMSConnectResourceType" => {
            // try to parse as CMSConnectResourceType
            let parsed: Result<CMSConnectResourceType, XmlError> = CMSConnectResourceType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtConvParticipantIntegDef" => {
            // try to parse as ExtConvParticipantIntegDef
            let parsed: Result<ExtConvParticipantIntegDef, XmlError> = ExtConvParticipantIntegDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExperienceBundleSettings" => {
            // try to parse as ExperienceBundleSettings
            let parsed: Result<ExperienceBundleSettings, XmlError> = ExperienceBundleSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Attachment" => {
            // try to parse as Attachment
            let parsed: Result<Attachment, XmlError> = Attachment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContractTypeConfig" => {
            // try to parse as ContractTypeConfig
            let parsed: Result<ContractTypeConfig, XmlError> = ContractTypeConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActnblListKeyPrfmIndDef" => {
            // try to parse as ActnblListKeyPrfmIndDef
            let parsed: Result<ActnblListKeyPrfmIndDef, XmlError> = ActnblListKeyPrfmIndDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ParticipantRole" => {
            // try to parse as ParticipantRole
            let parsed: Result<ParticipantRole, XmlError> = ParticipantRole::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PathAssistantSettings" => {
            // try to parse as PathAssistantSettings
            let parsed: Result<PathAssistantSettings, XmlError> = PathAssistantSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPluginInstructionDef" => {
            // try to parse as GenAiPluginInstructionDef
            let parsed: Result<GenAiPluginInstructionDef, XmlError> = GenAiPluginInstructionDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrTargetObjFieldMapping" => {
            // try to parse as OcrTargetObjFieldMapping
            let parsed: Result<OcrTargetObjFieldMapping, XmlError> = OcrTargetObjFieldMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StgFulfillmentStepDpndDef" => {
            // try to parse as StgFulfillmentStepDpndDef
            let parsed: Result<StgFulfillmentStepDpndDef, XmlError> = StgFulfillmentStepDpndDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WebLink" => {
            // try to parse as WebLink
            let parsed: Result<WebLink, XmlError> = WebLink::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ModuleDependencies" => {
            // try to parse as ModuleDependencies
            let parsed: Result<ModuleDependencies, XmlError> = ModuleDependencies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowDefinitionTranslation" => {
            // try to parse as FlowDefinitionTranslation
            let parsed: Result<FlowDefinitionTranslation, XmlError> = FlowDefinitionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedRecordAccessMap" => {
            // try to parse as RelatedRecordAccessMap
            let parsed: Result<RelatedRecordAccessMap, XmlError> = RelatedRecordAccessMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StandardValueSetTranslation" => {
            // try to parse as StandardValueSetTranslation
            let parsed: Result<StandardValueSetTranslation, XmlError> = StandardValueSetTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordTypePicklistValue" => {
            // try to parse as RecordTypePicklistValue
            let parsed: Result<RecordTypePicklistValue, XmlError> = RecordTypePicklistValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchOrgWideObjectConfig" => {
            // try to parse as SearchOrgWideObjectConfig
            let parsed: Result<SearchOrgWideObjectConfig, XmlError> = SearchOrgWideObjectConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Branding" => {
            // try to parse as Branding
            let parsed: Result<Branding, XmlError> = Branding::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionListItem" => {
            // try to parse as QuickActionListItem
            let parsed: Result<QuickActionListItem, XmlError> = QuickActionListItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorAttributeTranslation" => {
            // try to parse as DataConnectorAttributeTranslation
            let parsed: Result<DataConnectorAttributeTranslation, XmlError> = DataConnectorAttributeTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotStepTranslation" => {
            // try to parse as BotStepTranslation
            let parsed: Result<BotStepTranslation, XmlError> = BotStepTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ClauseCatgConfiguration" => {
            // try to parse as ClauseCatgConfiguration
            let parsed: Result<ClauseCatgConfiguration, XmlError> = ClauseCatgConfiguration::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActivationPlatform" => {
            // try to parse as ActivationPlatform
            let parsed: Result<ActivationPlatform, XmlError> = ActivationPlatform::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexipageDataSource" => {
            // try to parse as FlexipageDataSource
            let parsed: Result<FlexipageDataSource, XmlError> = FlexipageDataSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PriceRule" => {
            // try to parse as PriceRule
            let parsed: Result<PriceRule, XmlError> = PriceRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InvLatePymntRiskCalcSettings" => {
            // try to parse as InvLatePymntRiskCalcSettings
            let parsed: Result<InvLatePymntRiskCalcSettings, XmlError> = InvLatePymntRiskCalcSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniInteractionAccessConfig" => {
            // try to parse as OmniInteractionAccessConfig
            let parsed: Result<OmniInteractionAccessConfig, XmlError> = OmniInteractionAccessConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2Settings" => {
            // try to parse as Territory2Settings
            let parsed: Result<Territory2Settings, XmlError> = Territory2Settings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardFilter" => {
            // try to parse as DashboardFilter
            let parsed: Result<DashboardFilter, XmlError> = DashboardFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlModelArtifact" => {
            // try to parse as MlModelArtifact
            let parsed: Result<MlModelArtifact, XmlError> = MlModelArtifact::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IntegrationProviderAttr" => {
            // try to parse as IntegrationProviderAttr
            let parsed: Result<IntegrationProviderAttr, XmlError> = IntegrationProviderAttr::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TransactionProcessingType" => {
            // try to parse as TransactionProcessingType
            let parsed: Result<TransactionProcessingType, XmlError> = TransactionProcessingType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTestPoint" => {
            // try to parse as FlowTestPoint
            let parsed: Result<FlowTestPoint, XmlError> = FlowTestPoint::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowCoverageResult" => {
            // try to parse as FlowCoverageResult
            let parsed: Result<FlowCoverageResult, XmlError> = FlowCoverageResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AudienceCriteria" => {
            // try to parse as AudienceCriteria
            let parsed: Result<AudienceCriteria, XmlError> = AudienceCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileTabVisibility" => {
            // try to parse as ProfileTabVisibility
            let parsed: Result<ProfileTabVisibility, XmlError> = ProfileTabVisibility::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceFlowConfig" => {
            // try to parse as EmbeddedServiceFlowConfig
            let parsed: Result<EmbeddedServiceFlowConfig, XmlError> = EmbeddedServiceFlowConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VisualizationPlugin" => {
            // try to parse as VisualizationPlugin
            let parsed: Result<VisualizationPlugin, XmlError> = VisualizationPlugin::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "State" => {
            // try to parse as State
            let parsed: Result<State, XmlError> = State::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UiFormulaCriterion" => {
            // try to parse as UiFormulaCriterion
            let parsed: Result<UiFormulaCriterion, XmlError> = UiFormulaCriterion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PardotEinsteinSettings" => {
            // try to parse as PardotEinsteinSettings
            let parsed: Result<PardotEinsteinSettings, XmlError> = PardotEinsteinSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdDimensionCustomAction" => {
            // try to parse as WaveXmdDimensionCustomAction
            let parsed: Result<WaveXmdDimensionCustomAction, XmlError> = WaveXmdDimensionCustomAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordAlertTemplate" => {
            // try to parse as RecordAlertTemplate
            let parsed: Result<RecordAlertTemplate, XmlError> = RecordAlertTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceLiveAgent" => {
            // try to parse as EmbeddedServiceLiveAgent
            let parsed: Result<EmbeddedServiceLiveAgent, XmlError> = EmbeddedServiceLiveAgent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalAppIdTokenConfig" => {
            // try to parse as ExternalAppIdTokenConfig
            let parsed: Result<ExternalAppIdTokenConfig, XmlError> = ExternalAppIdTokenConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ItemInstance" => {
            // try to parse as ItemInstance
            let parsed: Result<ItemInstance, XmlError> = ItemInstance::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingCategoryMapping" => {
            // try to parse as ForecastingCategoryMapping
            let parsed: Result<ForecastingCategoryMapping, XmlError> = ForecastingCategoryMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ScoreCategoryCalcInsight" => {
            // try to parse as ScoreCategoryCalcInsight
            let parsed: Result<ScoreCategoryCalcInsight, XmlError> = ScoreCategoryCalcInsight::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorAttribute" => {
            // try to parse as DataConnectorAttribute
            let parsed: Result<DataConnectorAttribute, XmlError> = DataConnectorAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniTrackingGroup" => {
            // try to parse as OmniTrackingGroup
            let parsed: Result<OmniTrackingGroup, XmlError> = OmniTrackingGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ScoreCategory" => {
            // try to parse as ScoreCategory
            let parsed: Result<ScoreCategory, XmlError> = ScoreCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ComponentInstancePropertyListItem" => {
            // try to parse as ComponentInstancePropertyListItem
            let parsed: Result<ComponentInstancePropertyListItem, XmlError> = ComponentInstancePropertyListItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotInvocation" => {
            // try to parse as BotInvocation
            let parsed: Result<BotInvocation, XmlError> = BotInvocation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationVariable" => {
            // try to parse as ConversationVariable
            let parsed: Result<ConversationVariable, XmlError> = ConversationVariable::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommandAction" => {
            // try to parse as CommandAction
            let parsed: Result<CommandAction, XmlError> = CommandAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SubtabComponents" => {
            // try to parse as SubtabComponents
            let parsed: Result<SubtabComponents, XmlError> = SubtabComponents::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RealTimeEventSettings" => {
            // try to parse as RealTimeEventSettings
            let parsed: Result<RealTimeEventSettings, XmlError> = RealTimeEventSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIModelDefinition" => {
            // try to parse as AIModelDefinition
            let parsed: Result<AIModelDefinition, XmlError> = AIModelDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Users" => {
            // try to parse as Users
            let parsed: Result<Users, XmlError> = Users::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchSettings" => {
            // try to parse as SearchSettings
            let parsed: Result<SearchSettings, XmlError> = SearchSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InboundNetworkConnProperty" => {
            // try to parse as InboundNetworkConnProperty
            let parsed: Result<InboundNetworkConnProperty, XmlError> = InboundNetworkConnProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReleaseMgmtSettings" => {
            // try to parse as ReleaseMgmtSettings
            let parsed: Result<ReleaseMgmtSettings, XmlError> = ReleaseMgmtSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnblProgramTaskSubCategory" => {
            // try to parse as EnblProgramTaskSubCategory
            let parsed: Result<EnblProgramTaskSubCategory, XmlError> = EnblProgramTaskSubCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LwcResources" => {
            // try to parse as LwcResources
            let parsed: Result<LwcResources, XmlError> = LwcResources::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesContextSettings" => {
            // try to parse as IndustriesContextSettings
            let parsed: Result<IndustriesContextSettings, XmlError> = IndustriesContextSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesPricingSettings" => {
            // try to parse as IndustriesPricingSettings
            let parsed: Result<IndustriesPricingSettings, XmlError> = IndustriesPricingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MyDomainDiscoverableLogin" => {
            // try to parse as MyDomainDiscoverableLogin
            let parsed: Result<MyDomainDiscoverableLogin, XmlError> = MyDomainDiscoverableLogin::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionSendEmailOptions" => {
            // try to parse as QuickActionSendEmailOptions
            let parsed: Result<QuickActionSendEmailOptions, XmlError> = QuickActionSendEmailOptions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmailToCaseSettings" => {
            // try to parse as EmailToCaseSettings
            let parsed: Result<EmailToCaseSettings, XmlError> = EmailToCaseSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataObjectSearchIndexConf" => {
            // try to parse as DataObjectSearchIndexConf
            let parsed: Result<DataObjectSearchIndexConf, XmlError> = DataObjectSearchIndexConf::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ComponentInstance" => {
            // try to parse as ComponentInstance
            let parsed: Result<ComponentInstance, XmlError> = ComponentInstance::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WebToXSettings" => {
            // try to parse as WebToXSettings
            let parsed: Result<WebToXSettings, XmlError> = WebToXSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PostTemplate" => {
            // try to parse as PostTemplate
            let parsed: Result<PostTemplate, XmlError> = PostTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IframeWhiteListUrlSettings" => {
            // try to parse as IframeWhiteListUrlSettings
            let parsed: Result<IframeWhiteListUrlSettings, XmlError> = IframeWhiteListUrlSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogItemDefFiltrCrit" => {
            // try to parse as SvcCatalogItemDefFiltrCrit
            let parsed: Result<SvcCatalogItemDefFiltrCrit, XmlError> = SvcCatalogItemDefFiltrCrit::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniIntegrationProcedure" => {
            // try to parse as OmniIntegrationProcedure
            let parsed: Result<OmniIntegrationProcedure, XmlError> = OmniIntegrationProcedure::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniExtTrackingDef" => {
            // try to parse as OmniExtTrackingDef
            let parsed: Result<OmniExtTrackingDef, XmlError> = OmniExtTrackingDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordActionDeployment" => {
            // try to parse as RecordActionDeployment
            let parsed: Result<RecordActionDeployment, XmlError> = RecordActionDeployment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetDecisionTable" => {
            // try to parse as ExpressionSetDecisionTable
            let parsed: Result<ExpressionSetDecisionTable, XmlError> = ExpressionSetDecisionTable::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Roles" => {
            // try to parse as Roles
            let parsed: Result<Roles, XmlError> = Roles::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ScontrolTranslation" => {
            // try to parse as ScontrolTranslation
            let parsed: Result<ScontrolTranslation, XmlError> = ScontrolTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SettingOverride" => {
            // try to parse as SettingOverride
            let parsed: Result<SettingOverride, XmlError> = SettingOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Skill" => {
            // try to parse as Skill
            let parsed: Result<Skill, XmlError> = Skill::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IdentityVerificationProcDef" => {
            // try to parse as IdentityVerificationProcDef
            let parsed: Result<IdentityVerificationProcDef, XmlError> = IdentityVerificationProcDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PushNotification" => {
            // try to parse as PushNotification
            let parsed: Result<PushNotification, XmlError> = PushNotification::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CodeCoverageWarning" => {
            // try to parse as CodeCoverageWarning
            let parsed: Result<CodeCoverageWarning, XmlError> = CodeCoverageWarning::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "HomePageLayout" => {
            // try to parse as HomePageLayout
            let parsed: Result<HomePageLayout, XmlError> = HomePageLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardComponentSortInfo" => {
            // try to parse as DashboardComponentSortInfo
            let parsed: Result<DashboardComponentSortInfo, XmlError> = DashboardComponentSortInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiCoachAgentScnrDefTranslation" => {
            // try to parse as AiCoachAgentScnrDefTranslation
            let parsed: Result<AiCoachAgentScnrDefTranslation, XmlError> = AiCoachAgentScnrDefTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LocalMlDomain" => {
            // try to parse as LocalMlDomain
            let parsed: Result<LocalMlDomain, XmlError> = LocalMlDomain::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2RuleAssociation" => {
            // try to parse as Territory2RuleAssociation
            let parsed: Result<Territory2RuleAssociation, XmlError> = Territory2RuleAssociation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppSamlConfig" => {
            // try to parse as ConnectedAppSamlConfig
            let parsed: Result<ConnectedAppSamlConfig, XmlError> = ConnectedAppSamlConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceProcessItemGroup" => {
            // try to parse as ServiceProcessItemGroup
            let parsed: Result<ServiceProcessItemGroup, XmlError> = ServiceProcessItemGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MarketSegmentDefinition" => {
            // try to parse as MarketSegmentDefinition
            let parsed: Result<MarketSegmentDefinition, XmlError> = MarketSegmentDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveAgentSettings" => {
            // try to parse as LiveAgentSettings
            let parsed: Result<LiveAgentSettings, XmlError> = LiveAgentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowChoiceTranslation" => {
            // try to parse as FlowChoiceTranslation
            let parsed: Result<FlowChoiceTranslation, XmlError> = FlowChoiceTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Error" => {
            // try to parse as Error
            let parsed: Result<Error, XmlError> = Error::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConvReasonReportDefinition" => {
            // try to parse as ConvReasonReportDefinition
            let parsed: Result<ConvReasonReportDefinition, XmlError> = ConvReasonReportDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OauthCustomScopeApp" => {
            // try to parse as OauthCustomScopeApp
            let parsed: Result<OauthCustomScopeApp, XmlError> = OauthCustomScopeApp::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmailAdministrationSettings" => {
            // try to parse as EmailAdministrationSettings
            let parsed: Result<EmailAdministrationSettings, XmlError> = EmailAdministrationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportFilterItem" => {
            // try to parse as ReportFilterItem
            let parsed: Result<ReportFilterItem, XmlError> = ReportFilterItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DocumentGenerationSetting" => {
            // try to parse as DocumentGenerationSetting
            let parsed: Result<DocumentGenerationSetting, XmlError> = DocumentGenerationSetting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomNotificationType" => {
            // try to parse as CustomNotificationType
            let parsed: Result<CustomNotificationType, XmlError> = CustomNotificationType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChatterSettings" => {
            // try to parse as ChatterSettings
            let parsed: Result<ChatterSettings, XmlError> = ChatterSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DelegateGroup" => {
            // try to parse as DelegateGroup
            let parsed: Result<DelegateGroup, XmlError> = DelegateGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PackageTypeMembers" => {
            // try to parse as PackageTypeMembers
            let parsed: Result<PackageTypeMembers, XmlError> = PackageTypeMembers::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductConfiguratorSettings" => {
            // try to parse as ProductConfiguratorSettings
            let parsed: Result<ProductConfiguratorSettings, XmlError> = ProductConfiguratorSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SubscriptionManagementSettings" => {
            // try to parse as SubscriptionManagementSettings
            let parsed: Result<SubscriptionManagementSettings, XmlError> = SubscriptionManagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppOauthIpRange" => {
            // try to parse as ExtlClntAppOauthIpRange
            let parsed: Result<ExtlClntAppOauthIpRange, XmlError> = ExtlClntAppOauthIpRange::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MobileSecurityPolicy" => {
            // try to parse as MobileSecurityPolicy
            let parsed: Result<MobileSecurityPolicy, XmlError> = MobileSecurityPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AutoResponseRules" => {
            // try to parse as AutoResponseRules
            let parsed: Result<AutoResponseRules, XmlError> = AutoResponseRules::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharingBaseRule" => {
            // try to parse as SharingBaseRule
            let parsed: Result<SharingBaseRule, XmlError> = SharingBaseRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSpvsrConfigAIAgent" => {
            // try to parse as OmniSpvsrConfigAIAgent
            let parsed: Result<OmniSpvsrConfigAIAgent, XmlError> = OmniSpvsrConfigAIAgent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveChatButton" => {
            // try to parse as LiveChatButton
            let parsed: Result<LiveChatButton, XmlError> = LiveChatButton::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldInstance" => {
            // try to parse as FieldInstance
            let parsed: Result<FieldInstance, XmlError> = FieldInstance::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextUseCaseMapping" => {
            // try to parse as ContextUseCaseMapping
            let parsed: Result<ContextUseCaseMapping, XmlError> = ContextUseCaseMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportParam" => {
            // try to parse as ReportParam
            let parsed: Result<ReportParam, XmlError> = ReportParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationDefinitionStepGoalMapping" => {
            // try to parse as ConversationDefinitionStepGoalMapping
            let parsed: Result<ConversationDefinitionStepGoalMapping, XmlError> = ConversationDefinitionStepGoalMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestTopLevelWithCrud" => {
            // try to parse as FtestTopLevelWithCrud
            let parsed: Result<FtestTopLevelWithCrud, XmlError> = FtestTopLevelWithCrud::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldSourceTargetMap" => {
            // try to parse as FieldSourceTargetMap
            let parsed: Result<FieldSourceTargetMap, XmlError> = FieldSourceTargetMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniScript" => {
            // try to parse as OmniScript
            let parsed: Result<OmniScript, XmlError> = OmniScript::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataspaceScope" => {
            // try to parse as DataspaceScope
            let parsed: Result<DataspaceScope, XmlError> = DataspaceScope::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveTemplateBundle" => {
            // try to parse as WaveTemplateBundle
            let parsed: Result<WaveTemplateBundle, XmlError> = WaveTemplateBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AuraDefinition" => {
            // try to parse as AuraDefinition
            let parsed: Result<AuraDefinition, XmlError> = AuraDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NavigationMenuItemBranding" => {
            // try to parse as NavigationMenuItemBranding
            let parsed: Result<NavigationMenuItemBranding, XmlError> = NavigationMenuItemBranding::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PolicyRuleDefinitionCondition" => {
            // try to parse as PolicyRuleDefinitionCondition
            let parsed: Result<PolicyRuleDefinitionCondition, XmlError> = PolicyRuleDefinitionCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelationshipGraphDefVersion" => {
            // try to parse as RelationshipGraphDefVersion
            let parsed: Result<RelationshipGraphDefVersion, XmlError> = RelationshipGraphDefVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SystemNotificationSettings" => {
            // try to parse as SystemNotificationSettings
            let parsed: Result<SystemNotificationSettings, XmlError> = SystemNotificationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogCategoryItem" => {
            // try to parse as SvcCatalogCategoryItem
            let parsed: Result<SvcCatalogCategoryItem, XmlError> = SvcCatalogCategoryItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetAssignment" => {
            // try to parse as ExpressionSetAssignment
            let parsed: Result<ExpressionSetAssignment, XmlError> = ExpressionSetAssignment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetGroup" => {
            // try to parse as PermissionSetGroup
            let parsed: Result<PermissionSetGroup, XmlError> = PermissionSetGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileLoginHours" => {
            // try to parse as ProfileLoginHours
            let parsed: Result<ProfileLoginHours, XmlError> = ProfileLoginHours::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetAdvancedCondition" => {
            // try to parse as ExpressionSetAdvancedCondition
            let parsed: Result<ExpressionSetAdvancedCondition, XmlError> = ExpressionSetAdvancedCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OpportunityListFieldsLabelMapping" => {
            // try to parse as OpportunityListFieldsLabelMapping
            let parsed: Result<OpportunityListFieldsLabelMapping, XmlError> = OpportunityListFieldsLabelMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BusinessHoursEntry" => {
            // try to parse as BusinessHoursEntry
            let parsed: Result<BusinessHoursEntry, XmlError> = BusinessHoursEntry::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlRelatedIntent" => {
            // try to parse as MlRelatedIntent
            let parsed: Result<MlRelatedIntent, XmlError> = MlRelatedIntent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomHelpMenuItem" => {
            // try to parse as CustomHelpMenuItem
            let parsed: Result<CustomHelpMenuItem, XmlError> = CustomHelpMenuItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveXmdFormattingProperty" => {
            // try to parse as WaveXmdFormattingProperty
            let parsed: Result<WaveXmdFormattingProperty, XmlError> = WaveXmdFormattingProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetExternalCredentialPrincipalAccess" => {
            // try to parse as PermissionSetExternalCredentialPrincipalAccess
            let parsed: Result<PermissionSetExternalCredentialPrincipalAccess, XmlError> = PermissionSetExternalCredentialPrincipalAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OpportunityListFieldsSelectedSettings" => {
            // try to parse as OpportunityListFieldsSelectedSettings
            let parsed: Result<OpportunityListFieldsSelectedSettings, XmlError> = OpportunityListFieldsSelectedSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DefaultShortcut" => {
            // try to parse as DefaultShortcut
            let parsed: Result<DefaultShortcut, XmlError> = DefaultShortcut::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbdMsgChannelInvitationCondition" => {
            // try to parse as EmbdMsgChannelInvitationCondition
            let parsed: Result<EmbdMsgChannelInvitationCondition, XmlError> = EmbdMsgChannelInvitationCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotNavigationLink" => {
            // try to parse as BotNavigationLink
            let parsed: Result<BotNavigationLink, XmlError> = BotNavigationLink::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActvPfrmDataConnectorS3" => {
            // try to parse as ActvPfrmDataConnectorS3
            let parsed: Result<ActvPfrmDataConnectorS3, XmlError> = ActvPfrmDataConnectorS3::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetUserPermission" => {
            // try to parse as PermissionSetUserPermission
            let parsed: Result<PermissionSetUserPermission, XmlError> = PermissionSetUserPermission::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomFieldTranslation" => {
            // try to parse as CustomFieldTranslation
            let parsed: Result<CustomFieldTranslation, XmlError> = CustomFieldTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlannerAttr" => {
            // try to parse as GenAiPlannerAttr
            let parsed: Result<GenAiPlannerAttr, XmlError> = GenAiPlannerAttr::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CmsnStmtLineItemConfig" => {
            // try to parse as CmsnStmtLineItemConfig
            let parsed: Result<CmsnStmtLineItemConfig, XmlError> = CmsnStmtLineItemConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LearningAchievementConfig" => {
            // try to parse as LearningAchievementConfig
            let parsed: Result<LearningAchievementConfig, XmlError> = LearningAchievementConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CallCoachingMediaProvider" => {
            // try to parse as CallCoachingMediaProvider
            let parsed: Result<CallCoachingMediaProvider, XmlError> = CallCoachingMediaProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TelemetryDefinitionVersion" => {
            // try to parse as TelemetryDefinitionVersion
            let parsed: Result<TelemetryDefinitionVersion, XmlError> = TelemetryDefinitionVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CMSConnectAsset" => {
            // try to parse as CMSConnectAsset
            let parsed: Result<CMSConnectAsset, XmlError> = CMSConnectAsset::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetApexPageAccess" => {
            // try to parse as PermissionSetApexPageAccess
            let parsed: Result<PermissionSetApexPageAccess, XmlError> = PermissionSetApexPageAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelationshipGraphDefinition" => {
            // try to parse as RelationshipGraphDefinition
            let parsed: Result<RelationshipGraphDefinition, XmlError> = RelationshipGraphDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformActionList" => {
            // try to parse as PlatformActionList
            let parsed: Result<PlatformActionList, XmlError> = PlatformActionList::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IoTSettings" => {
            // try to parse as IoTSettings
            let parsed: Result<IoTSettings, XmlError> = IoTSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesChannelPartnerInventorySettings" => {
            // try to parse as IndustriesChannelPartnerInventorySettings
            let parsed: Result<IndustriesChannelPartnerInventorySettings, XmlError> = IndustriesChannelPartnerInventorySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileRecordTypeVisibility" => {
            // try to parse as ProfileRecordTypeVisibility
            let parsed: Result<ProfileRecordTypeVisibility, XmlError> = ProfileRecordTypeVisibility::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountPlanObjMeasCalcCond" => {
            // try to parse as AccountPlanObjMeasCalcCond
            let parsed: Result<AccountPlanObjMeasCalcCond, XmlError> = AccountPlanObjMeasCalcCond::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcessActionParameter" => {
            // try to parse as LoyaltyProgramProcessActionParameter
            let parsed: Result<LoyaltyProgramProcessActionParameter, XmlError> = LoyaltyProgramProcessActionParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FormulaSettings" => {
            // try to parse as FormulaSettings
            let parsed: Result<FormulaSettings, XmlError> = FormulaSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AutomatedContactsSettings" => {
            // try to parse as AutomatedContactsSettings
            let parsed: Result<AutomatedContactsSettings, XmlError> = AutomatedContactsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIManagedField" => {
            // try to parse as AIManagedField
            let parsed: Result<AIManagedField, XmlError> = AIManagedField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MobileApplicationDetail" => {
            // try to parse as MobileApplicationDetail
            let parsed: Result<MobileApplicationDetail, XmlError> = MobileApplicationDetail::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordActionDefaultItem" => {
            // try to parse as RecordActionDefaultItem
            let parsed: Result<RecordActionDefaultItem, XmlError> = RecordActionDefaultItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomHelpMenuSection" => {
            // try to parse as CustomHelpMenuSection
            let parsed: Result<CustomHelpMenuSection, XmlError> = CustomHelpMenuSection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BranchManagementSettings" => {
            // try to parse as BranchManagementSettings
            let parsed: Result<BranchManagementSettings, XmlError> = BranchManagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Holiday" => {
            // try to parse as Holiday
            let parsed: Result<Holiday, XmlError> = Holiday::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StageTransition" => {
            // try to parse as StageTransition
            let parsed: Result<StageTransition, XmlError> = StageTransition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionLayout" => {
            // try to parse as QuickActionLayout
            let parsed: Result<QuickActionLayout, XmlError> = QuickActionLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CorsWhitelistOrigin" => {
            // try to parse as CorsWhitelistOrigin
            let parsed: Result<CorsWhitelistOrigin, XmlError> = CorsWhitelistOrigin::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowOrchestrationStageTranslation" => {
            // try to parse as FlowOrchestrationStageTranslation
            let parsed: Result<FlowOrchestrationStageTranslation, XmlError> = FlowOrchestrationStageTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VisualizationResource" => {
            // try to parse as VisualizationResource
            let parsed: Result<VisualizationResource, XmlError> = VisualizationResource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PriceRuleConditionFilter" => {
            // try to parse as PriceRuleConditionFilter
            let parsed: Result<PriceRuleConditionFilter, XmlError> = PriceRuleConditionFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataStreamDefinition" => {
            // try to parse as DataStreamDefinition
            let parsed: Result<DataStreamDefinition, XmlError> = DataStreamDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CodeBuilderSettings" => {
            // try to parse as CodeBuilderSettings
            let parsed: Result<CodeBuilderSettings, XmlError> = CodeBuilderSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InterestTaggingSettings" => {
            // try to parse as InterestTaggingSettings
            let parsed: Result<InterestTaggingSettings, XmlError> = InterestTaggingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceMenuSettings" => {
            // try to parse as EmbeddedServiceMenuSettings
            let parsed: Result<EmbeddedServiceMenuSettings, XmlError> = EmbeddedServiceMenuSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CleanDataService" => {
            // try to parse as CleanDataService
            let parsed: Result<CleanDataService, XmlError> = CleanDataService::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformCachePartitionType" => {
            // try to parse as PlatformCachePartitionType
            let parsed: Result<PlatformCachePartitionType, XmlError> = PlatformCachePartitionType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomPermissionDependencyRequired" => {
            // try to parse as CustomPermissionDependencyRequired
            let parsed: Result<CustomPermissionDependencyRequired, XmlError> = CustomPermissionDependencyRequired::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApexTestSuite" => {
            // try to parse as ApexTestSuite
            let parsed: Result<ApexTestSuite, XmlError> = ApexTestSuite::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordActionDeploymentChannel" => {
            // try to parse as RecordActionDeploymentChannel
            let parsed: Result<RecordActionDeploymentChannel, XmlError> = RecordActionDeploymentChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalDataConnector" => {
            // try to parse as ExternalDataConnector
            let parsed: Result<ExternalDataConnector, XmlError> = ExternalDataConnector::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OnlineSalesSettings" => {
            // try to parse as OnlineSalesSettings
            let parsed: Result<OnlineSalesSettings, XmlError> = OnlineSalesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickAction" => {
            // try to parse as QuickAction
            let parsed: Result<QuickAction, XmlError> = QuickAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppSampleConfigurablePolicies" => {
            // try to parse as ExtlClntAppSampleConfigurablePolicies
            let parsed: Result<ExtlClntAppSampleConfigurablePolicies, XmlError> = ExtlClntAppSampleConfigurablePolicies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommissionStatementConfig" => {
            // try to parse as CommissionStatementConfig
            let parsed: Result<CommissionStatementConfig, XmlError> = CommissionStatementConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiEvaluationTestCaseCritParam" => {
            // try to parse as AiEvaluationTestCaseCritParam
            let parsed: Result<AiEvaluationTestCaseCritParam, XmlError> = AiEvaluationTestCaseCritParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharingRules" => {
            // try to parse as SharingRules
            let parsed: Result<SharingRules, XmlError> = SharingRules::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotVersionTranslation" => {
            // try to parse as BotVersionTranslation
            let parsed: Result<BotVersionTranslation, XmlError> = BotVersionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TelemetryDefinition" => {
            // try to parse as TelemetryDefinition
            let parsed: Result<TelemetryDefinition, XmlError> = TelemetryDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PresenceUserConfig" => {
            // try to parse as PresenceUserConfig
            let parsed: Result<PresenceUserConfig, XmlError> = PresenceUserConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeAnswerSettings" => {
            // try to parse as KnowledgeAnswerSettings
            let parsed: Result<KnowledgeAnswerSettings, XmlError> = KnowledgeAnswerSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTestAssertion" => {
            // try to parse as FlowTestAssertion
            let parsed: Result<FlowTestAssertion, XmlError> = FlowTestAssertion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIFactorComponent" => {
            // try to parse as AIFactorComponent
            let parsed: Result<AIFactorComponent, XmlError> = AIFactorComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportCrossFilter" => {
            // try to parse as ReportCrossFilter
            let parsed: Result<ReportCrossFilter, XmlError> = ReportCrossFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BriefcaseRuleFilter" => {
            // try to parse as BriefcaseRuleFilter
            let parsed: Result<BriefcaseRuleFilter, XmlError> = BriefcaseRuleFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReputationPointsRule" => {
            // try to parse as ReputationPointsRule
            let parsed: Result<ReputationPointsRule, XmlError> = ReputationPointsRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogItemAttribute" => {
            // try to parse as SvcCatalogItemAttribute
            let parsed: Result<SvcCatalogItemAttribute, XmlError> = SvcCatalogItemAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Picklist" => {
            // try to parse as Picklist
            let parsed: Result<Picklist, XmlError> = Picklist::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DgtAssetMgmtProvider" => {
            // try to parse as DgtAssetMgmtProvider
            let parsed: Result<DgtAssetMgmtProvider, XmlError> = DgtAssetMgmtProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChannelLayoutItem" => {
            // try to parse as ChannelLayoutItem
            let parsed: Result<ChannelLayoutItem, XmlError> = ChannelLayoutItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UIObjectRelationConfig" => {
            // try to parse as UIObjectRelationConfig
            let parsed: Result<UIObjectRelationConfig, XmlError> = UIObjectRelationConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DxGlobalTermsSettings" => {
            // try to parse as DxGlobalTermsSettings
            let parsed: Result<DxGlobalTermsSettings, XmlError> = DxGlobalTermsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionTable" => {
            // try to parse as DecisionTable
            let parsed: Result<DecisionTable, XmlError> = DecisionTable::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfilePasswordPolicy" => {
            // try to parse as ProfilePasswordPolicy
            let parsed: Result<ProfilePasswordPolicy, XmlError> = ProfilePasswordPolicy::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AuraDefinitions" => {
            // try to parse as AuraDefinitions
            let parsed: Result<AuraDefinitions, XmlError> = AuraDefinitions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CaseSubjectParticle" => {
            // try to parse as CaseSubjectParticle
            let parsed: Result<CaseSubjectParticle, XmlError> = CaseSubjectParticle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DisclosureType" => {
            // try to parse as DisclosureType
            let parsed: Result<DisclosureType, XmlError> = DisclosureType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldRestrictionRule" => {
            // try to parse as FieldRestrictionRule
            let parsed: Result<FieldRestrictionRule, XmlError> = FieldRestrictionRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MiniLayout" => {
            // try to parse as MiniLayout
            let parsed: Result<MiniLayout, XmlError> = MiniLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InventorySettings" => {
            // try to parse as InventorySettings
            let parsed: Result<InventorySettings, XmlError> = InventorySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppOauthPoliciesAttribute" => {
            // try to parse as ExtlClntAppOauthPoliciesAttribute
            let parsed: Result<ExtlClntAppOauthPoliciesAttribute, XmlError> = ExtlClntAppOauthPoliciesAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MobileSecurityAssignment" => {
            // try to parse as MobileSecurityAssignment
            let parsed: Result<MobileSecurityAssignment, XmlError> = MobileSecurityAssignment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProcedureOutputResolution" => {
            // try to parse as ProcedureOutputResolution
            let parsed: Result<ProcedureOutputResolution, XmlError> = ProcedureOutputResolution::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalAuthIdentityProvider" => {
            // try to parse as ExternalAuthIdentityProvider
            let parsed: Result<ExternalAuthIdentityProvider, XmlError> = ExternalAuthIdentityProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataPackageKitDefinition" => {
            // try to parse as DataPackageKitDefinition
            let parsed: Result<DataPackageKitDefinition, XmlError> = DataPackageKitDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionableListMemberStatus" => {
            // try to parse as ActionableListMemberStatus
            let parsed: Result<ActionableListMemberStatus, XmlError> = ActionableListMemberStatus::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotVariableOperationTranslation" => {
            // try to parse as BotVariableOperationTranslation
            let parsed: Result<BotVariableOperationTranslation, XmlError> = BotVariableOperationTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CareLimitType" => {
            // try to parse as CareLimitType
            let parsed: Result<CareLimitType, XmlError> = CareLimitType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IdentityVerificationProcFld" => {
            // try to parse as IdentityVerificationProcFld
            let parsed: Result<IdentityVerificationProcFld, XmlError> = IdentityVerificationProcFld::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InvocableActionExtension" => {
            // try to parse as InvocableActionExtension
            let parsed: Result<InvocableActionExtension, XmlError> = InvocableActionExtension::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesEventOrchSettings" => {
            // try to parse as IndustriesEventOrchSettings
            let parsed: Result<IndustriesEventOrchSettings, XmlError> = IndustriesEventOrchSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordAlertDataSource" => {
            // try to parse as RecordAlertDataSource
            let parsed: Result<RecordAlertDataSource, XmlError> = RecordAlertDataSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationDefinitionPlanner" => {
            // try to parse as ConversationDefinitionPlanner
            let parsed: Result<ConversationDefinitionPlanner, XmlError> = ConversationDefinitionPlanner::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationDefinitionChannelProvider" => {
            // try to parse as ConversationDefinitionChannelProvider
            let parsed: Result<ConversationDefinitionChannelProvider, XmlError> = ConversationDefinitionChannelProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LwcResource" => {
            // try to parse as LwcResource
            let parsed: Result<LwcResource, XmlError> = LwcResource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DeleteResult" => {
            // try to parse as DeleteResult
            let parsed: Result<DeleteResult, XmlError> = DeleteResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomApplicationTranslation" => {
            // try to parse as CustomApplicationTranslation
            let parsed: Result<CustomApplicationTranslation, XmlError> = CustomApplicationTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageLayoutCompositeValue" => {
            // try to parse as ConversationMessageLayoutCompositeValue
            let parsed: Result<ConversationMessageLayoutCompositeValue, XmlError> = ConversationMessageLayoutCompositeValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorError" => {
            // try to parse as DataConnectorError
            let parsed: Result<DataConnectorError, XmlError> = DataConnectorError::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExplainabilityActionVersion" => {
            // try to parse as ExplainabilityActionVersion
            let parsed: Result<ExplainabilityActionVersion, XmlError> = ExplainabilityActionVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogCategory" => {
            // try to parse as SvcCatalogCategory
            let parsed: Result<SvcCatalogCategory, XmlError> = SvcCatalogCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountForecastSettings" => {
            // try to parse as AccountForecastSettings
            let parsed: Result<AccountForecastSettings, XmlError> = AccountForecastSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniStudioSettings" => {
            // try to parse as OmniStudioSettings
            let parsed: Result<OmniStudioSettings, XmlError> = OmniStudioSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AudienceContactPoint" => {
            // try to parse as AudienceContactPoint
            let parsed: Result<AudienceContactPoint, XmlError> = AudienceContactPoint::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppOauthSettingsAttribute" => {
            // try to parse as ExtlClntAppOauthSettingsAttribute
            let parsed: Result<ExtlClntAppOauthSettingsAttribute, XmlError> = ExtlClntAppOauthSettingsAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingKeyword" => {
            // try to parse as MessagingKeyword
            let parsed: Result<MessagingKeyword, XmlError> = MessagingKeyword::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowScreenFieldTranslation" => {
            // try to parse as FlowScreenFieldTranslation
            let parsed: Result<FlowScreenFieldTranslation, XmlError> = FlowScreenFieldTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowChoiceUserInputTranslation" => {
            // try to parse as FlowChoiceUserInputTranslation
            let parsed: Result<FlowChoiceUserInputTranslation, XmlError> = FlowChoiceUserInputTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MLGenerativeDefinition" => {
            // try to parse as MLGenerativeDefinition
            let parsed: Result<MLGenerativeDefinition, XmlError> = MLGenerativeDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EinsteinGptSettings" => {
            // try to parse as EinsteinGptSettings
            let parsed: Result<EinsteinGptSettings, XmlError> = EinsteinGptSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkDotComSettings" => {
            // try to parse as WorkDotComSettings
            let parsed: Result<WorkDotComSettings, XmlError> = WorkDotComSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Index" => {
            // try to parse as Index
            let parsed: Result<Index, XmlError> = Index::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CatalogedApiInstance" => {
            // try to parse as CatalogedApiInstance
            let parsed: Result<CatalogedApiInstance, XmlError> = CatalogedApiInstance::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryStoryOutcome" => {
            // try to parse as DiscoveryStoryOutcome
            let parsed: Result<DiscoveryStoryOutcome, XmlError> = DiscoveryStoryOutcome::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SlackRecordLayout" => {
            // try to parse as SlackRecordLayout
            let parsed: Result<SlackRecordLayout, XmlError> = SlackRecordLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniInteractionConfig" => {
            // try to parse as OmniInteractionConfig
            let parsed: Result<OmniInteractionConfig, XmlError> = OmniInteractionConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIFilterValue" => {
            // try to parse as AIFilterValue
            let parsed: Result<AIFilterValue, XmlError> = AIFilterValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingChannelCustomParameter" => {
            // try to parse as MessagingChannelCustomParameter
            let parsed: Result<MessagingChannelCustomParameter, XmlError> = MessagingChannelCustomParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceCustomization" => {
            // try to parse as EmbeddedServiceCustomization
            let parsed: Result<EmbeddedServiceCustomization, XmlError> = EmbeddedServiceCustomization::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CompanySettings" => {
            // try to parse as CompanySettings
            let parsed: Result<CompanySettings, XmlError> = CompanySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningBoltImages" => {
            // try to parse as LightningBoltImages
            let parsed: Result<LightningBoltImages, XmlError> = LightningBoltImages::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIModelFactor" => {
            // try to parse as AIModelFactor
            let parsed: Result<AIModelFactor, XmlError> = AIModelFactor::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityThemeRouteOverride" => {
            // try to parse as CommunityThemeRouteOverride
            let parsed: Result<CommunityThemeRouteOverride, XmlError> = CommunityThemeRouteOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataCalcInsightTemplate" => {
            // try to parse as DataCalcInsightTemplate
            let parsed: Result<DataCalcInsightTemplate, XmlError> = DataCalcInsightTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataDotComSettings" => {
            // try to parse as DataDotComSettings
            let parsed: Result<DataDotComSettings, XmlError> = DataDotComSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IncludedPlatformLicenseDefinition" => {
            // try to parse as IncludedPlatformLicenseDefinition
            let parsed: Result<IncludedPlatformLicenseDefinition, XmlError> = IncludedPlatformLicenseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataCategory" => {
            // try to parse as DataCategory
            let parsed: Result<DataCategory, XmlError> = DataCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppOauthAssetToken" => {
            // try to parse as ConnectedAppOauthAssetToken
            let parsed: Result<ConnectedAppOauthAssetToken, XmlError> = ConnectedAppOauthAssetToken::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PasswordPolicies" => {
            // try to parse as PasswordPolicies
            let parsed: Result<PasswordPolicies, XmlError> = PasswordPolicies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowStageTranslation" => {
            // try to parse as FlowStageTranslation
            let parsed: Result<FlowStageTranslation, XmlError> = FlowStageTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommerceSettings" => {
            // try to parse as CommerceSettings
            let parsed: Result<CommerceSettings, XmlError> = CommerceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIUsecaseFieldMapping" => {
            // try to parse as AIUsecaseFieldMapping
            let parsed: Result<AIUsecaseFieldMapping, XmlError> = AIUsecaseFieldMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Package" => {
            // try to parse as Package
            let parsed: Result<Package, XmlError> = Package::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionPlanTemplateItemDependency" => {
            // try to parse as ActionPlanTemplateItemDependency
            let parsed: Result<ActionPlanTemplateItemDependency, XmlError> = ActionPlanTemplateItemDependency::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunicationChannelType" => {
            // try to parse as CommunicationChannelType
            let parsed: Result<CommunicationChannelType, XmlError> = CommunicationChannelType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordType" => {
            // try to parse as RecordType
            let parsed: Result<RecordType, XmlError> = RecordType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PricingProcedureOutputMap" => {
            // try to parse as PricingProcedureOutputMap
            let parsed: Result<PricingProcedureOutputMap, XmlError> = PricingProcedureOutputMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceBranding" => {
            // try to parse as EmbeddedServiceBranding
            let parsed: Result<EmbeddedServiceBranding, XmlError> = EmbeddedServiceBranding::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StreamingAppDataConnector" => {
            // try to parse as StreamingAppDataConnector
            let parsed: Result<StreamingAppDataConnector, XmlError> = StreamingAppDataConnector::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationDefinition" => {
            // try to parse as RecommendationDefinition
            let parsed: Result<RecommendationDefinition, XmlError> = RecommendationDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ListView" => {
            // try to parse as ListView
            let parsed: Result<ListView, XmlError> = ListView::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContentSettings" => {
            // try to parse as ContentSettings
            let parsed: Result<ContentSettings, XmlError> = ContentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestDetailWithDeclMd1" => {
            // try to parse as FtestDetailWithDeclMd1
            let parsed: Result<FtestDetailWithDeclMd1, XmlError> = FtestDetailWithDeclMd1::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MarketAudienceField" => {
            // try to parse as MarketAudienceField
            let parsed: Result<MarketAudienceField, XmlError> = MarketAudienceField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDatalakeSrcKeyQualifier" => {
            // try to parse as MktDatalakeSrcKeyQualifier
            let parsed: Result<MktDatalakeSrcKeyQualifier, XmlError> = MktDatalakeSrcKeyQualifier::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfigTab" => {
            // try to parse as OmniSupervisorConfigTab
            let parsed: Result<OmniSupervisorConfigTab, XmlError> = OmniSupervisorConfigTab::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityRoles" => {
            // try to parse as CommunityRoles
            let parsed: Result<CommunityRoles, XmlError> = CommunityRoles::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobFrcstGrpFld" => {
            // try to parse as BatchCalcJobFrcstGrpFld
            let parsed: Result<BatchCalcJobFrcstGrpFld, XmlError> = BatchCalcJobFrcstGrpFld::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomConsoleComponents" => {
            // try to parse as CustomConsoleComponents
            let parsed: Result<CustomConsoleComponents, XmlError> = CustomConsoleComponents::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogFulfillmentFlow" => {
            // try to parse as SvcCatalogFulfillmentFlow
            let parsed: Result<SvcCatalogFulfillmentFlow, XmlError> = SvcCatalogFulfillmentFlow::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DescribeValueTypeResult" => {
            // try to parse as DescribeValueTypeResult
            let parsed: Result<DescribeValueTypeResult, XmlError> = DescribeValueTypeResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardComponentSection" => {
            // try to parse as DashboardComponentSection
            let parsed: Result<DashboardComponentSection, XmlError> = DashboardComponentSection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GlobalValueSetTranslation" => {
            // try to parse as GlobalValueSetTranslation
            let parsed: Result<GlobalValueSetTranslation, XmlError> = GlobalValueSetTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ManagedContentTypeBundle" => {
            // try to parse as ManagedContentTypeBundle
            let parsed: Result<ManagedContentTypeBundle, XmlError> = ManagedContentTypeBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowTestParameter" => {
            // try to parse as FlowTestParameter
            let parsed: Result<FlowTestParameter, XmlError> = FlowTestParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionsSettings" => {
            // try to parse as ActionsSettings
            let parsed: Result<ActionsSettings, XmlError> = ActionsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomLabelTranslation" => {
            // try to parse as CustomLabelTranslation
            let parsed: Result<CustomLabelTranslation, XmlError> = CustomLabelTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssessmentConfiguration" => {
            // try to parse as AssessmentConfiguration
            let parsed: Result<AssessmentConfiguration, XmlError> = AssessmentConfiguration::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RegisteredExternalService" => {
            // try to parse as RegisteredExternalService
            let parsed: Result<RegisteredExternalService, XmlError> = RegisteredExternalService::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowScreenTranslation" => {
            // try to parse as FlowScreenTranslation
            let parsed: Result<FlowScreenTranslation, XmlError> = FlowScreenTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesUnifiedInventorySettings" => {
            // try to parse as IndustriesUnifiedInventorySettings
            let parsed: Result<IndustriesUnifiedInventorySettings, XmlError> = IndustriesUnifiedInventorySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppExperienceSettings" => {
            // try to parse as AppExperienceSettings
            let parsed: Result<AppExperienceSettings, XmlError> = AppExperienceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldOverride" => {
            // try to parse as FieldOverride
            let parsed: Result<FieldOverride, XmlError> = FieldOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InsRatePlanCmsnConfig" => {
            // try to parse as InsRatePlanCmsnConfig
            let parsed: Result<InsRatePlanCmsnConfig, XmlError> = InsRatePlanCmsnConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PardotSettings" => {
            // try to parse as PardotSettings
            let parsed: Result<PardotSettings, XmlError> = PardotSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataLakeFieldAttributes" => {
            // try to parse as MktDataLakeFieldAttributes
            let parsed: Result<MktDataLakeFieldAttributes, XmlError> = MktDataLakeFieldAttributes::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReputationLevelDefinitions" => {
            // try to parse as ReputationLevelDefinitions
            let parsed: Result<ReputationLevelDefinitions, XmlError> = ReputationLevelDefinitions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantContextItem" => {
            // try to parse as AssistantContextItem
            let parsed: Result<AssistantContextItem, XmlError> = AssistantContextItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IncidentMgmtSettings" => {
            // try to parse as IncidentMgmtSettings
            let parsed: Result<IncidentMgmtSettings, XmlError> = IncidentMgmtSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningOnboardingConfig" => {
            // try to parse as LightningOnboardingConfig
            let parsed: Result<LightningOnboardingConfig, XmlError> = LightningOnboardingConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CMSConnectLanguage" => {
            // try to parse as CMSConnectLanguage
            let parsed: Result<CMSConnectLanguage, XmlError> = CMSConnectLanguage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WeightedSourceCategory" => {
            // try to parse as WeightedSourceCategory
            let parsed: Result<WeightedSourceCategory, XmlError> = WeightedSourceCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldValue" => {
            // try to parse as FieldValue
            let parsed: Result<FieldValue, XmlError> = FieldValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AppAnalyticsSettings" => {
            // try to parse as AppAnalyticsSettings
            let parsed: Result<AppAnalyticsSettings, XmlError> = AppAnalyticsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PredictionBuilderSettings" => {
            // try to parse as PredictionBuilderSettings
            let parsed: Result<PredictionBuilderSettings, XmlError> = PredictionBuilderSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LicenseDefinition" => {
            // try to parse as LicenseDefinition
            let parsed: Result<LicenseDefinition, XmlError> = LicenseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceProcess" => {
            // try to parse as ServiceProcess
            let parsed: Result<ServiceProcess, XmlError> = ServiceProcess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorS3" => {
            // try to parse as DataConnectorS3
            let parsed: Result<DataConnectorS3, XmlError> = DataConnectorS3::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportTypeColumnTranslation" => {
            // try to parse as ReportTypeColumnTranslation
            let parsed: Result<ReportTypeColumnTranslation, XmlError> = ReportTypeColumnTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PrimaryTabComponents" => {
            // try to parse as PrimaryTabComponents
            let parsed: Result<PrimaryTabComponents, XmlError> = PrimaryTabComponents::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IdentityVerificationProcDtl" => {
            // try to parse as IdentityVerificationProcDtl
            let parsed: Result<IdentityVerificationProcDtl, XmlError> = IdentityVerificationProcDtl::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DgtAssetMgmtPrvdLghtCpnt" => {
            // try to parse as DgtAssetMgmtPrvdLghtCpnt
            let parsed: Result<DgtAssetMgmtPrvdLghtCpnt, XmlError> = DgtAssetMgmtPrvdLghtCpnt::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogFilterCriteria" => {
            // try to parse as SvcCatalogFilterCriteria
            let parsed: Result<SvcCatalogFilterCriteria, XmlError> = SvcCatalogFilterCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldMappingRow" => {
            // try to parse as FieldMappingRow
            let parsed: Result<FieldMappingRow, XmlError> = FieldMappingRow::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IdeaReputationLevel" => {
            // try to parse as IdeaReputationLevel
            let parsed: Result<IdeaReputationLevel, XmlError> = IdeaReputationLevel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OauthCustomScope" => {
            // try to parse as OauthCustomScope
            let parsed: Result<OauthCustomScope, XmlError> = OauthCustomScope::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIModel" => {
            // try to parse as AIModel
            let parsed: Result<AIModel, XmlError> = AIModel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReferralMarketingSettings" => {
            // try to parse as ReferralMarketingSettings
            let parsed: Result<ReferralMarketingSettings, XmlError> = ReferralMarketingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestFirstTopLevel" => {
            // try to parse as FtestFirstTopLevel
            let parsed: Result<FtestFirstTopLevel, XmlError> = FtestFirstTopLevel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationContextVariable" => {
            // try to parse as ConversationContextVariable
            let parsed: Result<ConversationContextVariable, XmlError> = ConversationContextVariable::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileLoginIpRange" => {
            // try to parse as ProfileLoginIpRange
            let parsed: Result<ProfileLoginIpRange, XmlError> = ProfileLoginIpRange::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmployeeFieldAccessSettings" => {
            // try to parse as EmployeeFieldAccessSettings
            let parsed: Result<EmployeeFieldAccessSettings, XmlError> = EmployeeFieldAccessSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WebLinkTranslation" => {
            // try to parse as WebLinkTranslation
            let parsed: Result<WebLinkTranslation, XmlError> = WebLinkTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MilestoneCompletionCriteria" => {
            // try to parse as MilestoneCompletionCriteria
            let parsed: Result<MilestoneCompletionCriteria, XmlError> = MilestoneCompletionCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningBolt" => {
            // try to parse as LightningBolt
            let parsed: Result<LightningBolt, XmlError> = LightningBolt::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StageAssignment" => {
            // try to parse as StageAssignment
            let parsed: Result<StageAssignment, XmlError> = StageAssignment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningTypeBundle" => {
            // try to parse as LightningTypeBundle
            let parsed: Result<LightningTypeBundle, XmlError> = LightningTypeBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetVariable" => {
            // try to parse as ExpressionSetVariable
            let parsed: Result<ExpressionSetVariable, XmlError> = ExpressionSetVariable::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceFieldService" => {
            // try to parse as EmbeddedServiceFieldService
            let parsed: Result<EmbeddedServiceFieldService, XmlError> = EmbeddedServiceFieldService::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DigitalExperienceBundle" => {
            // try to parse as DigitalExperienceBundle
            let parsed: Result<DigitalExperienceBundle, XmlError> = DigitalExperienceBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageConstantValueTranslation" => {
            // try to parse as ConversationMessageConstantValueTranslation
            let parsed: Result<ConversationMessageConstantValueTranslation, XmlError> = ConversationMessageConstantValueTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IfExpression" => {
            // try to parse as IfExpression
            let parsed: Result<IfExpression, XmlError> = IfExpression::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharedWith" => {
            // try to parse as SharedWith
            let parsed: Result<SharedWith, XmlError> = SharedWith::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomObjectBinding" => {
            // try to parse as CustomObjectBinding
            let parsed: Result<CustomObjectBinding, XmlError> = CustomObjectBinding::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrSampleDocumentField" => {
            // try to parse as OcrSampleDocumentField
            let parsed: Result<OcrSampleDocumentField, XmlError> = OcrSampleDocumentField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StandardFieldTranslation" => {
            // try to parse as StandardFieldTranslation
            let parsed: Result<StandardFieldTranslation, XmlError> = StandardFieldTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ManagedEventSubscription" => {
            // try to parse as ManagedEventSubscription
            let parsed: Result<ManagedEventSubscription, XmlError> = ManagedEventSubscription::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomNotificationActionGroup" => {
            // try to parse as CustomNotificationActionGroup
            let parsed: Result<CustomNotificationActionGroup, XmlError> = CustomNotificationActionGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WarrantyLifecycleMgmtSettings" => {
            // try to parse as WarrantyLifecycleMgmtSettings
            let parsed: Result<WarrantyLifecycleMgmtSettings, XmlError> = WarrantyLifecycleMgmtSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingSettings" => {
            // try to parse as ForecastingSettings
            let parsed: Result<ForecastingSettings, XmlError> = ForecastingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AgentConfigButtons" => {
            // try to parse as AgentConfigButtons
            let parsed: Result<AgentConfigButtons, XmlError> = AgentConfigButtons::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ArticleTypeTemplate" => {
            // try to parse as ArticleTypeTemplate
            let parsed: Result<ArticleTypeTemplate, XmlError> = ArticleTypeTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2SettingsOpportunityFilter" => {
            // try to parse as Territory2SettingsOpportunityFilter
            let parsed: Result<Territory2SettingsOpportunityFilter, XmlError> = Territory2SettingsOpportunityFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetCustomPermissions" => {
            // try to parse as PermissionSetCustomPermissions
            let parsed: Result<PermissionSetCustomPermissions, XmlError> = PermissionSetCustomPermissions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordActionDeploymentContext" => {
            // try to parse as RecordActionDeploymentContext
            let parsed: Result<RecordActionDeploymentContext, XmlError> = RecordActionDeploymentContext::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionLauncherItemDef" => {
            // try to parse as ActionLauncherItemDef
            let parsed: Result<ActionLauncherItemDef, XmlError> = ActionLauncherItemDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SalesAccountAgentSettings" => {
            // try to parse as SalesAccountAgentSettings
            let parsed: Result<SalesAccountAgentSettings, XmlError> = SalesAccountAgentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EACSettings" => {
            // try to parse as EACSettings
            let parsed: Result<EACSettings, XmlError> = EACSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NetworkMemberGroup" => {
            // try to parse as NetworkMemberGroup
            let parsed: Result<NetworkMemberGroup, XmlError> = NetworkMemberGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WorkflowFlowActionParameter" => {
            // try to parse as WorkflowFlowActionParameter
            let parsed: Result<WorkflowFlowActionParameter, XmlError> = WorkflowFlowActionParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardGridComponent" => {
            // try to parse as DashboardGridComponent
            let parsed: Result<DashboardGridComponent, XmlError> = DashboardGridComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationAudience" => {
            // try to parse as RecommendationAudience
            let parsed: Result<RecommendationAudience, XmlError> = RecommendationAudience::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldInstanceProperty" => {
            // try to parse as FieldInstanceProperty
            let parsed: Result<FieldInstanceProperty, XmlError> = FieldInstanceProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomApplicationComponent" => {
            // try to parse as CustomApplicationComponent
            let parsed: Result<CustomApplicationComponent, XmlError> = CustomApplicationComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeCaseFieldsSettings" => {
            // try to parse as KnowledgeCaseFieldsSettings
            let parsed: Result<KnowledgeCaseFieldsSettings, XmlError> = KnowledgeCaseFieldsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommandActionParam" => {
            // try to parse as CommandActionParam
            let parsed: Result<CommandActionParam, XmlError> = CommandActionParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountingModelConfig" => {
            // try to parse as AccountingModelConfig
            let parsed: Result<AccountingModelConfig, XmlError> = AccountingModelConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataCleanRoomProvider" => {
            // try to parse as DataCleanRoomProvider
            let parsed: Result<DataCleanRoomProvider, XmlError> = DataCleanRoomProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowCoverageWarning" => {
            // try to parse as FlowCoverageWarning
            let parsed: Result<FlowCoverageWarning, XmlError> = FlowCoverageWarning::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VehicleAssetEmssnSrcCnfg" => {
            // try to parse as VehicleAssetEmssnSrcCnfg
            let parsed: Result<VehicleAssetEmssnSrcCnfg, XmlError> = VehicleAssetEmssnSrcCnfg::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MfgProgramTemplate" => {
            // try to parse as MfgProgramTemplate
            let parsed: Result<MfgProgramTemplate, XmlError> = MfgProgramTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Metadata" => {
            // try to parse as Metadata
            let parsed: Result<Metadata, XmlError> = Metadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LayoutItem" => {
            // try to parse as LayoutItem
            let parsed: Result<LayoutItem, XmlError> = LayoutItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationRecordLookupField" => {
            // try to parse as ConversationRecordLookupField
            let parsed: Result<ConversationRecordLookupField, XmlError> = ConversationRecordLookupField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformSlackSettings" => {
            // try to parse as PlatformSlackSettings
            let parsed: Result<PlatformSlackSettings, XmlError> = PlatformSlackSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionTableDatasetLink" => {
            // try to parse as DecisionTableDatasetLink
            let parsed: Result<DecisionTableDatasetLink, XmlError> = DecisionTableDatasetLink::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrSampleDocumentPageItem" => {
            // try to parse as OcrSampleDocumentPageItem
            let parsed: Result<OcrSampleDocumentPageItem, XmlError> = OcrSampleDocumentPageItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppAttribute" => {
            // try to parse as ConnectedAppAttribute
            let parsed: Result<ConnectedAppAttribute, XmlError> = ConnectedAppAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowComplexLiteralTranslation" => {
            // try to parse as FlowComplexLiteralTranslation
            let parsed: Result<FlowComplexLiteralTranslation, XmlError> = FlowComplexLiteralTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AgentConfigSkills" => {
            // try to parse as AgentConfigSkills
            let parsed: Result<AgentConfigSkills, XmlError> = AgentConfigSkills::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StrategyNodeInvocableActionArg" => {
            // try to parse as StrategyNodeInvocableActionArg
            let parsed: Result<StrategyNodeInvocableActionArg, XmlError> = StrategyNodeInvocableActionArg::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Profile" => {
            // try to parse as Profile
            let parsed: Result<Profile, XmlError> = Profile::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageEventTargetProperty" => {
            // try to parse as FlexiPageEventTargetProperty
            let parsed: Result<FlexiPageEventTargetProperty, XmlError> = FlexiPageEventTargetProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveChatSensitiveDataRule" => {
            // try to parse as LiveChatSensitiveDataRule
            let parsed: Result<LiveChatSensitiveDataRule, XmlError> = LiveChatSensitiveDataRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LocalizedValue" => {
            // try to parse as LocalizedValue
            let parsed: Result<LocalizedValue, XmlError> = LocalizedValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductSpecificationType" => {
            // try to parse as ProductSpecificationType
            let parsed: Result<ProductSpecificationType, XmlError> = ProductSpecificationType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NtfcnCondition" => {
            // try to parse as NtfcnCondition
            let parsed: Result<NtfcnCondition, XmlError> = NtfcnCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingFilterCondition" => {
            // try to parse as ForecastingFilterCondition
            let parsed: Result<ForecastingFilterCondition, XmlError> = ForecastingFilterCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIModelMetric" => {
            // try to parse as AIModelMetric
            let parsed: Result<AIModelMetric, XmlError> = AIModelMetric::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniTrackingComponentDef" => {
            // try to parse as OmniTrackingComponentDef
            let parsed: Result<OmniTrackingComponentDef, XmlError> = OmniTrackingComponentDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AccountRelationshipShareRule" => {
            // try to parse as AccountRelationshipShareRule
            let parsed: Result<AccountRelationshipShareRule, XmlError> = AccountRelationshipShareRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetExternalDataSourceAccess" => {
            // try to parse as PermissionSetExternalDataSourceAccess
            let parsed: Result<PermissionSetExternalDataSourceAccess, XmlError> = PermissionSetExternalDataSourceAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotBlockVersionTranslation" => {
            // try to parse as BotBlockVersionTranslation
            let parsed: Result<BotBlockVersionTranslation, XmlError> = BotBlockVersionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrTargetObject" => {
            // try to parse as OcrTargetObject
            let parsed: Result<OcrTargetObject, XmlError> = OcrTargetObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmpSvcInvocableActionDef" => {
            // try to parse as EmpSvcInvocableActionDef
            let parsed: Result<EmpSvcInvocableActionDef, XmlError> = EmpSvcInvocableActionDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotBlock" => {
            // try to parse as BotBlock
            let parsed: Result<BotBlock, XmlError> = BotBlock::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportChart" => {
            // try to parse as ReportChart
            let parsed: Result<ReportChart, XmlError> = ReportChart::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AiEvaluationDefinition" => {
            // try to parse as AiEvaluationDefinition
            let parsed: Result<AiEvaluationDefinition, XmlError> = AiEvaluationDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetMessageToken" => {
            // try to parse as ExpressionSetMessageToken
            let parsed: Result<ExpressionSetMessageToken, XmlError> = ExpressionSetMessageToken::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UiFormatSpecification" => {
            // try to parse as UiFormatSpecification
            let parsed: Result<UiFormatSpecification, XmlError> = UiFormatSpecification::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LargeQuotesandOrdersForRlmSettings" => {
            // try to parse as LargeQuotesandOrdersForRlmSettings
            let parsed: Result<LargeQuotesandOrdersForRlmSettings, XmlError> = LargeQuotesandOrdersForRlmSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BriefcaseRule" => {
            // try to parse as BriefcaseRule
            let parsed: Result<BriefcaseRule, XmlError> = BriefcaseRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileCustomMetadataTypeAccess" => {
            // try to parse as ProfileCustomMetadataTypeAccess
            let parsed: Result<ProfileCustomMetadataTypeAccess, XmlError> = ProfileCustomMetadataTypeAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChatterExtension" => {
            // try to parse as ChatterExtension
            let parsed: Result<ChatterExtension, XmlError> = ChatterExtension::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantSkillSobjectAction" => {
            // try to parse as AssistantSkillSobjectAction
            let parsed: Result<AssistantSkillSobjectAction, XmlError> = AssistantSkillSobjectAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ManagedTopics" => {
            // try to parse as ManagedTopics
            let parsed: Result<ManagedTopics, XmlError> = ManagedTopics::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProcessFlowMigration" => {
            // try to parse as ProcessFlowMigration
            let parsed: Result<ProcessFlowMigration, XmlError> = ProcessFlowMigration::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetStep" => {
            // try to parse as ExpressionSetStep
            let parsed: Result<ExpressionSetStep, XmlError> = ExpressionSetStep::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedApp" => {
            // try to parse as ConnectedApp
            let parsed: Result<ConnectedApp, XmlError> = ConnectedApp::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExperienceResource" => {
            // try to parse as ExperienceResource
            let parsed: Result<ExperienceResource, XmlError> = ExperienceResource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PaymentGatewayProvider" => {
            // try to parse as PaymentGatewayProvider
            let parsed: Result<PaymentGatewayProvider, XmlError> = PaymentGatewayProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CloudServiceProvider" => {
            // try to parse as CloudServiceProvider
            let parsed: Result<CloudServiceProvider, XmlError> = CloudServiceProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MLDataDefinition" => {
            // try to parse as MLDataDefinition
            let parsed: Result<MLDataDefinition, XmlError> = MLDataDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchDataSourceOrderField" => {
            // try to parse as BatchDataSourceOrderField
            let parsed: Result<BatchDataSourceOrderField, XmlError> = BatchDataSourceOrderField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventRelayConfig" => {
            // try to parse as EventRelayConfig
            let parsed: Result<EventRelayConfig, XmlError> = EventRelayConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NamedCredentialParameter" => {
            // try to parse as NamedCredentialParameter
            let parsed: Result<NamedCredentialParameter, XmlError> = NamedCredentialParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmailServicesAddress" => {
            // try to parse as EmailServicesAddress
            let parsed: Result<EmailServicesAddress, XmlError> = EmailServicesAddress::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotStep" => {
            // try to parse as BotStep
            let parsed: Result<BotStep, XmlError> = BotStep::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PrivacySettings" => {
            // try to parse as PrivacySettings
            let parsed: Result<PrivacySettings, XmlError> = PrivacySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeCaseSettings" => {
            // try to parse as KnowledgeCaseSettings
            let parsed: Result<KnowledgeCaseSettings, XmlError> = KnowledgeCaseSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NameSettings" => {
            // try to parse as NameSettings
            let parsed: Result<NameSettings, XmlError> = NameSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FileTypeDispositionAssignmentBean" => {
            // try to parse as FileTypeDispositionAssignmentBean
            let parsed: Result<FileTypeDispositionAssignmentBean, XmlError> = FileTypeDispositionAssignmentBean::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExperienceContainer" => {
            // try to parse as ExperienceContainer
            let parsed: Result<ExperienceContainer, XmlError> = ExperienceContainer::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordActionSelectableItem" => {
            // try to parse as RecordActionSelectableItem
            let parsed: Result<RecordActionSelectableItem, XmlError> = RecordActionSelectableItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageDefinitionTranslation" => {
            // try to parse as ConversationMessageDefinitionTranslation
            let parsed: Result<ConversationMessageDefinitionTranslation, XmlError> = ConversationMessageDefinitionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NetworkTabSet" => {
            // try to parse as NetworkTabSet
            let parsed: Result<NetworkTabSet, XmlError> = NetworkTabSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldMappingField" => {
            // try to parse as FieldMappingField
            let parsed: Result<FieldMappingField, XmlError> = FieldMappingField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningComponentBundle" => {
            // try to parse as LightningComponentBundle
            let parsed: Result<LightningComponentBundle, XmlError> = LightningComponentBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GoogleAppsSettings" => {
            // try to parse as GoogleAppsSettings
            let parsed: Result<GoogleAppsSettings, XmlError> = GoogleAppsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingChannelStandardParameter" => {
            // try to parse as MessagingChannelStandardParameter
            let parsed: Result<MessagingChannelStandardParameter, XmlError> = MessagingChannelStandardParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CareBenefitVerifySettings" => {
            // try to parse as CareBenefitVerifySettings
            let parsed: Result<CareBenefitVerifySettings, XmlError> = CareBenefitVerifySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CareRequestRecords" => {
            // try to parse as CareRequestRecords
            let parsed: Result<CareRequestRecords, XmlError> = CareRequestRecords::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardMobileSettings" => {
            // try to parse as DashboardMobileSettings
            let parsed: Result<DashboardMobileSettings, XmlError> = DashboardMobileSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIScoringModelDefVersion" => {
            // try to parse as AIScoringModelDefVersion
            let parsed: Result<AIScoringModelDefVersion, XmlError> = AIScoringModelDefVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InvocableActionExtensionTarget" => {
            // try to parse as InvocableActionExtensionTarget
            let parsed: Result<InvocableActionExtensionTarget, XmlError> = InvocableActionExtensionTarget::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformActionListItem" => {
            // try to parse as PlatformActionListItem
            let parsed: Result<PlatformActionListItem, XmlError> = PlatformActionListItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningExperienceSettings" => {
            // try to parse as LightningExperienceSettings
            let parsed: Result<LightningExperienceSettings, XmlError> = LightningExperienceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceChannel" => {
            // try to parse as ServiceChannel
            let parsed: Result<ServiceChannel, XmlError> = ServiceChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SkillType" => {
            // try to parse as SkillType
            let parsed: Result<SkillType, XmlError> = SkillType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EscalationAction" => {
            // try to parse as EscalationAction
            let parsed: Result<EscalationAction, XmlError> = EscalationAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "VoiceEngmtMediaFileAsgnt" => {
            // try to parse as VoiceEngmtMediaFileAsgnt
            let parsed: Result<VoiceEngmtMediaFileAsgnt, XmlError> = VoiceEngmtMediaFileAsgnt::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetConditionExpression" => {
            // try to parse as ExpressionSetConditionExpression
            let parsed: Result<ExpressionSetConditionExpression, XmlError> = ExpressionSetConditionExpression::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BusinessProcessDefinition" => {
            // try to parse as BusinessProcessDefinition
            let parsed: Result<BusinessProcessDefinition, XmlError> = BusinessProcessDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FtestDetailWithDeclMd2" => {
            // try to parse as FtestDetailWithDeclMd2
            let parsed: Result<FtestDetailWithDeclMd2, XmlError> = FtestDetailWithDeclMd2::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PolicyRuleDefinition" => {
            // try to parse as PolicyRuleDefinition
            let parsed: Result<PolicyRuleDefinition, XmlError> = PolicyRuleDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniChannelSettings" => {
            // try to parse as OmniChannelSettings
            let parsed: Result<OmniChannelSettings, XmlError> = OmniChannelSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementMeasureRelatedObjectDefinition" => {
            // try to parse as EnablementMeasureRelatedObjectDefinition
            let parsed: Result<EnablementMeasureRelatedObjectDefinition, XmlError> = EnablementMeasureRelatedObjectDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SecuritySettings" => {
            // try to parse as SecuritySettings
            let parsed: Result<SecuritySettings, XmlError> = SecuritySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchCustomizationObjectOverride" => {
            // try to parse as SearchCustomizationObjectOverride
            let parsed: Result<SearchCustomizationObjectOverride, XmlError> = SearchCustomizationObjectOverride::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningMessageChannel" => {
            // try to parse as LightningMessageChannel
            let parsed: Result<LightningMessageChannel, XmlError> = LightningMessageChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApexSettings" => {
            // try to parse as ApexSettings
            let parsed: Result<ApexSettings, XmlError> = ApexSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmployeeDataSyncProfile" => {
            // try to parse as EmployeeDataSyncProfile
            let parsed: Result<EmployeeDataSyncProfile, XmlError> = EmployeeDataSyncProfile::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CMSConnectPersonalization" => {
            // try to parse as CMSConnectPersonalization
            let parsed: Result<CMSConnectPersonalization, XmlError> = CMSConnectPersonalization::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityTemplateBundleInfo" => {
            // try to parse as CommunityTemplateBundleInfo
            let parsed: Result<CommunityTemplateBundleInfo, XmlError> = CommunityTemplateBundleInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardGridLayout" => {
            // try to parse as DashboardGridLayout
            let parsed: Result<DashboardGridLayout, XmlError> = DashboardGridLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NtfcnCriteria" => {
            // try to parse as NtfcnCriteria
            let parsed: Result<NtfcnCriteria, XmlError> = NtfcnCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExpressionSetSubExpression" => {
            // try to parse as ExpressionSetSubExpression
            let parsed: Result<ExpressionSetSubExpression, XmlError> = ExpressionSetSubExpression::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickTextSettings" => {
            // try to parse as QuickTextSettings
            let parsed: Result<QuickTextSettings, XmlError> = QuickTextSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageLayoutPrimitiveValue" => {
            // try to parse as ConversationMessageLayoutPrimitiveValue
            let parsed: Result<ConversationMessageLayoutPrimitiveValue, XmlError> = ConversationMessageLayoutPrimitiveValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SettingValue" => {
            // try to parse as SettingValue
            let parsed: Result<SettingValue, XmlError> = SettingValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectionParamTmpl" => {
            // try to parse as DataConnectionParamTmpl
            let parsed: Result<DataConnectionParamTmpl, XmlError> = DataConnectionParamTmpl::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "McpServerToolDefinition" => {
            // try to parse as McpServerToolDefinition
            let parsed: Result<McpServerToolDefinition, XmlError> = McpServerToolDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalDataSource" => {
            // try to parse as ExternalDataSource
            let parsed: Result<ExternalDataSource, XmlError> = ExternalDataSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApiSchemaTypes" => {
            // try to parse as ApiSchemaTypes
            let parsed: Result<ApiSchemaTypes, XmlError> = ApiSchemaTypes::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppSamlConfigurablePoliciesAttribute" => {
            // try to parse as ExtlClntAppSamlConfigurablePoliciesAttribute
            let parsed: Result<ExtlClntAppSamlConfigurablePoliciesAttribute, XmlError> = ExtlClntAppSamlConfigurablePoliciesAttribute::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InsPlcyCoverageSpecConfig" => {
            // try to parse as InsPlcyCoverageSpecConfig
            let parsed: Result<InsPlcyCoverageSpecConfig, XmlError> = InsPlcyCoverageSpecConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeedLayoutComponent" => {
            // try to parse as FeedLayoutComponent
            let parsed: Result<FeedLayoutComponent, XmlError> = FeedLayoutComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunicationChannelLine" => {
            // try to parse as CommunicationChannelLine
            let parsed: Result<CommunicationChannelLine, XmlError> = CommunicationChannelLine::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LetterheadLine" => {
            // try to parse as LetterheadLine
            let parsed: Result<LetterheadLine, XmlError> = LetterheadLine::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OpportunitySettings" => {
            // try to parse as OpportunitySettings
            let parsed: Result<OpportunitySettings, XmlError> = OpportunitySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveChatDeployment" => {
            // try to parse as LiveChatDeployment
            let parsed: Result<LiveChatDeployment, XmlError> = LiveChatDeployment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Territory2AccessLevel" => {
            // try to parse as Territory2AccessLevel
            let parsed: Result<Territory2AccessLevel, XmlError> = Territory2AccessLevel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlModelOutputEndpoint" => {
            // try to parse as MlModelOutputEndpoint
            let parsed: Result<MlModelOutputEndpoint, XmlError> = MlModelOutputEndpoint::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TimeSheetTemplate" => {
            // try to parse as TimeSheetTemplate
            let parsed: Result<TimeSheetTemplate, XmlError> = TimeSheetTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceChannelStatusFieldMapping" => {
            // try to parse as ServiceChannelStatusFieldMapping
            let parsed: Result<ServiceChannelStatusFieldMapping, XmlError> = ServiceChannelStatusFieldMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RelatedList" => {
            // try to parse as RelatedList
            let parsed: Result<RelatedList, XmlError> = RelatedList::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContentAssetLink" => {
            // try to parse as ContentAssetLink
            let parsed: Result<ContentAssetLink, XmlError> = ContentAssetLink::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppGlobalOauthSettings" => {
            // try to parse as ExtlClntAppGlobalOauthSettings
            let parsed: Result<ExtlClntAppGlobalOauthSettings, XmlError> = ExtlClntAppGlobalOauthSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CMSConnectResourceDefinition" => {
            // try to parse as CMSConnectResourceDefinition
            let parsed: Result<CMSConnectResourceDefinition, XmlError> = CMSConnectResourceDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetApexClassAccess" => {
            // try to parse as PermissionSetApexClassAccess
            let parsed: Result<PermissionSetApexClassAccess, XmlError> = PermissionSetApexClassAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataConnectorAttributeOptTranslation" => {
            // try to parse as DataConnectorAttributeOptTranslation
            let parsed: Result<DataConnectorAttributeOptTranslation, XmlError> = DataConnectorAttributeOptTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardFilterOption" => {
            // try to parse as DashboardFilterOption
            let parsed: Result<DashboardFilterOption, XmlError> = DashboardFilterOption::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DigitalExperienceFolderShare" => {
            // try to parse as DigitalExperienceFolderShare
            let parsed: Result<DigitalExperienceFolderShare, XmlError> = DigitalExperienceFolderShare::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlModelConnection" => {
            // try to parse as MlModelConnection
            let parsed: Result<MlModelConnection, XmlError> = MlModelConnection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TelemetryActnDefStepAttr" => {
            // try to parse as TelemetryActnDefStepAttr
            let parsed: Result<TelemetryActnDefStepAttr, XmlError> = TelemetryActnDefStepAttr::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataPlatDataSetBundle" => {
            // try to parse as DataPlatDataSetBundle
            let parsed: Result<DataPlatDataSetBundle, XmlError> = DataPlatDataSetBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReputationPointsRules" => {
            // try to parse as ReputationPointsRules
            let parsed: Result<ReputationPointsRules, XmlError> = ReputationPointsRules::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EnablementProgramTaskCmsContent" => {
            // try to parse as EnablementProgramTaskCmsContent
            let parsed: Result<EnablementProgramTaskCmsContent, XmlError> = EnablementProgramTaskCmsContent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SharingRecalculation" => {
            // try to parse as SharingRecalculation
            let parsed: Result<SharingRecalculation, XmlError> = SharingRecalculation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileCategoryGroupVisibility" => {
            // try to parse as ProfileCategoryGroupVisibility
            let parsed: Result<ProfileCategoryGroupVisibility, XmlError> = ProfileCategoryGroupVisibility::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextNode" => {
            // try to parse as ContextNode
            let parsed: Result<ContextNode, XmlError> = ContextNode::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageDefinition" => {
            // try to parse as ConversationMessageDefinition
            let parsed: Result<ConversationMessageDefinition, XmlError> = ConversationMessageDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowIcon" => {
            // try to parse as FlowIcon
            let parsed: Result<FlowIcon, XmlError> = FlowIcon::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CallCenterRoutingMap" => {
            // try to parse as CallCenterRoutingMap
            let parsed: Result<CallCenterRoutingMap, XmlError> = CallCenterRoutingMap::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextAttrHydrationDetail" => {
            // try to parse as ContextAttrHydrationDetail
            let parsed: Result<ContextAttrHydrationDetail, XmlError> = ContextAttrHydrationDetail::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Dashboard" => {
            // try to parse as Dashboard
            let parsed: Result<Dashboard, XmlError> = Dashboard::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomApplication" => {
            // try to parse as CustomApplication
            let parsed: Result<CustomApplication, XmlError> = CustomApplication::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldServiceSettings" => {
            // try to parse as FieldServiceSettings
            let parsed: Result<FieldServiceSettings, XmlError> = FieldServiceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DeployOptions" => {
            // try to parse as DeployOptions
            let parsed: Result<DeployOptions, XmlError> = DeployOptions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionableEventTypeDef" => {
            // try to parse as ActionableEventTypeDef
            let parsed: Result<ActionableEventTypeDef, XmlError> = ActionableEventTypeDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIModelGraph" => {
            // try to parse as AIModelGraph
            let parsed: Result<AIModelGraph, XmlError> = AIModelGraph::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportFormattingRuleValue" => {
            // try to parse as ReportFormattingRuleValue
            let parsed: Result<ReportFormattingRuleValue, XmlError> = ReportFormattingRuleValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportTypeTranslation" => {
            // try to parse as ReportTypeTranslation
            let parsed: Result<ReportTypeTranslation, XmlError> = ReportTypeTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SkillProfileAssignments" => {
            // try to parse as SkillProfileAssignments
            let parsed: Result<SkillProfileAssignments, XmlError> = SkillProfileAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportColumn" => {
            // try to parse as ReportColumn
            let parsed: Result<ReportColumn, XmlError> = ReportColumn::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldSetItem" => {
            // try to parse as FieldSetItem
            let parsed: Result<FieldSetItem, XmlError> = FieldSetItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CspTrustedSite" => {
            // try to parse as CspTrustedSite
            let parsed: Result<CspTrustedSite, XmlError> = CspTrustedSite::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LayoutSectionTranslation" => {
            // try to parse as LayoutSectionTranslation
            let parsed: Result<LayoutSectionTranslation, XmlError> = LayoutSectionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReputationLevels" => {
            // try to parse as ReputationLevels
            let parsed: Result<ReputationLevels, XmlError> = ReputationLevels::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageParameterCompositeDetails" => {
            // try to parse as ConversationMessageParameterCompositeDetails
            let parsed: Result<ConversationMessageParameterCompositeDetails, XmlError> = ConversationMessageParameterCompositeDetails::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantSkill" => {
            // try to parse as AssistantSkill
            let parsed: Result<AssistantSkill, XmlError> = AssistantSkill::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProductDiscoverySettings" => {
            // try to parse as ProductDiscoverySettings
            let parsed: Result<ProductDiscoverySettings, XmlError> = ProductDiscoverySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataConnectionParam" => {
            // try to parse as MktDataConnectionParam
            let parsed: Result<MktDataConnectionParam, XmlError> = MktDataConnectionParam::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniAssessmentTaskMetadata" => {
            // try to parse as OmniAssessmentTaskMetadata
            let parsed: Result<OmniAssessmentTaskMetadata, XmlError> = OmniAssessmentTaskMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DescribeMetadataResult" => {
            // try to parse as DescribeMetadataResult
            let parsed: Result<DescribeMetadataResult, XmlError> = DescribeMetadataResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BusinessProcess" => {
            // try to parse as BusinessProcess
            let parsed: Result<BusinessProcess, XmlError> = BusinessProcess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CallCenterItem" => {
            // try to parse as CallCenterItem
            let parsed: Result<CallCenterItem, XmlError> = CallCenterItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalConversationBotDef" => {
            // try to parse as ExternalConversationBotDef
            let parsed: Result<ExternalConversationBotDef, XmlError> = ExternalConversationBotDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RulesetDefinition" => {
            // try to parse as RulesetDefinition
            let parsed: Result<RulesetDefinition, XmlError> = RulesetDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingType" => {
            // try to parse as ForecastingType
            let parsed: Result<ForecastingType, XmlError> = ForecastingType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileObjectPermissions" => {
            // try to parse as ProfileObjectPermissions
            let parsed: Result<ProfileObjectPermissions, XmlError> = ProfileObjectPermissions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BlacklistedConsumer" => {
            // try to parse as BlacklistedConsumer
            let parsed: Result<BlacklistedConsumer, XmlError> = BlacklistedConsumer::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SamlSsoConfig" => {
            // try to parse as SamlSsoConfig
            let parsed: Result<SamlSsoConfig, XmlError> = SamlSsoConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Workflow" => {
            // try to parse as Workflow
            let parsed: Result<Workflow, XmlError> = Workflow::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalCredential" => {
            // try to parse as ExternalCredential
            let parsed: Result<ExternalCredential, XmlError> = ExternalCredential::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KeywordList" => {
            // try to parse as KeywordList
            let parsed: Result<KeywordList, XmlError> = KeywordList::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingSubmissionSettings" => {
            // try to parse as ForecastingSubmissionSettings
            let parsed: Result<ForecastingSubmissionSettings, XmlError> = ForecastingSubmissionSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ValueSetValuesDefinition" => {
            // try to parse as ValueSetValuesDefinition
            let parsed: Result<ValueSetValuesDefinition, XmlError> = ValueSetValuesDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiFunction" => {
            // try to parse as GenAiFunction
            let parsed: Result<GenAiFunction, XmlError> = GenAiFunction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ProfileSearchLayouts" => {
            // try to parse as ProfileSearchLayouts
            let parsed: Result<ProfileSearchLayouts, XmlError> = ProfileSearchLayouts::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrSampleDocument" => {
            // try to parse as OcrSampleDocument
            let parsed: Result<OcrSampleDocument, XmlError> = OcrSampleDocument::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportBucketField" => {
            // try to parse as ReportBucketField
            let parsed: Result<ReportBucketField, XmlError> = ReportBucketField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MetadataGroup" => {
            // try to parse as MetadataGroup
            let parsed: Result<MetadataGroup, XmlError> = MetadataGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomizablePropensityScoringSettings" => {
            // try to parse as CustomizablePropensityScoringSettings
            let parsed: Result<CustomizablePropensityScoringSettings, XmlError> = CustomizablePropensityScoringSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DecisionMatrixDefinitionVersion" => {
            // try to parse as DecisionMatrixDefinitionVersion
            let parsed: Result<DecisionMatrixDefinitionVersion, XmlError> = DecisionMatrixDefinitionVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionTranslation" => {
            // try to parse as QuickActionTranslation
            let parsed: Result<QuickActionTranslation, XmlError> = QuickActionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceLayoutRule" => {
            // try to parse as EmbeddedServiceLayoutRule
            let parsed: Result<EmbeddedServiceLayoutRule, XmlError> = EmbeddedServiceLayoutRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalDocStorageConfig" => {
            // try to parse as ExternalDocStorageConfig
            let parsed: Result<ExternalDocStorageConfig, XmlError> = ExternalDocStorageConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniUiCard" => {
            // try to parse as OmniUiCard
            let parsed: Result<OmniUiCard, XmlError> = OmniUiCard::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantSkillIntent" => {
            // try to parse as AssistantSkillIntent
            let parsed: Result<AssistantSkillIntent, XmlError> = AssistantSkillIntent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformCachePartition" => {
            // try to parse as PlatformCachePartition
            let parsed: Result<PlatformCachePartition, XmlError> = PlatformCachePartition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CareRequestConfiguration" => {
            // try to parse as CareRequestConfiguration
            let parsed: Result<CareRequestConfiguration, XmlError> = CareRequestConfiguration::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UiFormulaRule" => {
            // try to parse as UiFormulaRule
            let parsed: Result<UiFormulaRule, XmlError> = UiFormulaRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationDefinitionRichMessage" => {
            // try to parse as ConversationDefinitionRichMessage
            let parsed: Result<ConversationDefinitionRichMessage, XmlError> = ConversationDefinitionRichMessage::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "McpServerPromptDefinition" => {
            // try to parse as McpServerPromptDefinition
            let parsed: Result<McpServerPromptDefinition, XmlError> = McpServerPromptDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAcctForecastDimSource" => {
            // try to parse as AdvAcctForecastDimSource
            let parsed: Result<AdvAcctForecastDimSource, XmlError> = AdvAcctForecastDimSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OpptStageDescription" => {
            // try to parse as OpptStageDescription
            let parsed: Result<OpptStageDescription, XmlError> = OpptStageDescription::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LifeSciConfigRecord" => {
            // try to parse as LifeSciConfigRecord
            let parsed: Result<LifeSciConfigRecord, XmlError> = LifeSciConfigRecord::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppOauthSettings" => {
            // try to parse as ExtlClntAppOauthSettings
            let parsed: Result<ExtlClntAppOauthSettings, XmlError> = ExtlClntAppOauthSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageCompSchemaPropertyDef" => {
            // try to parse as FlexiPageCompSchemaPropertyDef
            let parsed: Result<FlexiPageCompSchemaPropertyDef, XmlError> = FlexiPageCompSchemaPropertyDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GlobalQuickActionTranslation" => {
            // try to parse as GlobalQuickActionTranslation
            let parsed: Result<GlobalQuickActionTranslation, XmlError> = GlobalQuickActionTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedMessagingChannel" => {
            // try to parse as EmbeddedMessagingChannel
            let parsed: Result<EmbeddedMessagingChannel, XmlError> = EmbeddedMessagingChannel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LookupFilterTranslation" => {
            // try to parse as LookupFilterTranslation
            let parsed: Result<LookupFilterTranslation, XmlError> = LookupFilterTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SummaryLayout" => {
            // try to parse as SummaryLayout
            let parsed: Result<SummaryLayout, XmlError> = SummaryLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotDialog" => {
            // try to parse as BotDialog
            let parsed: Result<BotDialog, XmlError> = BotDialog::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StageCondition" => {
            // try to parse as StageCondition
            let parsed: Result<StageCondition, XmlError> = StageCondition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QueueRoutingConfigSkill" => {
            // try to parse as QueueRoutingConfigSkill
            let parsed: Result<QueueRoutingConfigSkill, XmlError> = QueueRoutingConfigSkill::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContractType" => {
            // try to parse as ContractType
            let parsed: Result<ContractType, XmlError> = ContractType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserCriteria" => {
            // try to parse as UserCriteria
            let parsed: Result<UserCriteria, XmlError> = UserCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchCalcJobDefinition" => {
            // try to parse as BatchCalcJobDefinition
            let parsed: Result<BatchCalcJobDefinition, XmlError> = BatchCalcJobDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowFerovTranslation" => {
            // try to parse as FlowFerovTranslation
            let parsed: Result<FlowFerovTranslation, XmlError> = FlowFerovTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChatterAnswersSettings" => {
            // try to parse as ChatterAnswersSettings
            let parsed: Result<ChatterAnswersSettings, XmlError> = ChatterAnswersSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfig" => {
            // try to parse as OmniSupervisorConfig
            let parsed: Result<OmniSupervisorConfig, XmlError> = OmniSupervisorConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BusinessProcessTypeDefinition" => {
            // try to parse as BusinessProcessTypeDefinition
            let parsed: Result<BusinessProcessTypeDefinition, XmlError> = BusinessProcessTypeDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataObjectBuildOrgTemplate" => {
            // try to parse as DataObjectBuildOrgTemplate
            let parsed: Result<DataObjectBuildOrgTemplate, XmlError> = DataObjectBuildOrgTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WaveTemplateExternalDataMetadata" => {
            // try to parse as WaveTemplateExternalDataMetadata
            let parsed: Result<WaveTemplateExternalDataMetadata, XmlError> = WaveTemplateExternalDataMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryGoal" => {
            // try to parse as DiscoveryGoal
            let parsed: Result<DiscoveryGoal, XmlError> = DiscoveryGoal::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsDashPageWidget" => {
            // try to parse as AnalyticsDashPageWidget
            let parsed: Result<AnalyticsDashPageWidget, XmlError> = AnalyticsDashPageWidget::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeSuggestedArticlesSettings" => {
            // try to parse as KnowledgeSuggestedArticlesSettings
            let parsed: Result<KnowledgeSuggestedArticlesSettings, XmlError> = KnowledgeSuggestedArticlesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OutboundNetworkConnection" => {
            // try to parse as OutboundNetworkConnection
            let parsed: Result<OutboundNetworkConnection, XmlError> = OutboundNetworkConnection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetApplicationVisibility" => {
            // try to parse as PermissionSetApplicationVisibility
            let parsed: Result<PermissionSetApplicationVisibility, XmlError> = PermissionSetApplicationVisibility::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExternalAuthIdentityProviderParameter" => {
            // try to parse as ExternalAuthIdentityProviderParameter
            let parsed: Result<ExternalAuthIdentityProviderParameter, XmlError> = ExternalAuthIdentityProviderParameter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MilestoneType" => {
            // try to parse as MilestoneType
            let parsed: Result<MilestoneType, XmlError> = MilestoneType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtDataTranFieldTemplate" => {
            // try to parse as ExtDataTranFieldTemplate
            let parsed: Result<ExtDataTranFieldTemplate, XmlError> = ExtDataTranFieldTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SupervisorAgentConfigSkills" => {
            // try to parse as SupervisorAgentConfigSkills
            let parsed: Result<SupervisorAgentConfigSkills, XmlError> = SupervisorAgentConfigSkills::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UpdateMetadata" => {
            // try to parse as UpdateMetadata
            let parsed: Result<UpdateMetadata, XmlError> = UpdateMetadata::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BatchProcessJobDefinition" => {
            // try to parse as BatchProcessJobDefinition
            let parsed: Result<BatchProcessJobDefinition, XmlError> = BatchProcessJobDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataPackageKitObject" => {
            // try to parse as DataPackageKitObject
            let parsed: Result<DataPackageKitObject, XmlError> = DataPackageKitObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PortalDelegablePermissionSet" => {
            // try to parse as PortalDelegablePermissionSet
            let parsed: Result<PortalDelegablePermissionSet, XmlError> = PortalDelegablePermissionSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LookupFilter" => {
            // try to parse as LookupFilter
            let parsed: Result<LookupFilter, XmlError> = LookupFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsCloudComponentLayoutItem" => {
            // try to parse as AnalyticsCloudComponentLayoutItem
            let parsed: Result<AnalyticsCloudComponentLayoutItem, XmlError> = AnalyticsCloudComponentLayoutItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchSettingsByObject" => {
            // try to parse as SearchSettingsByObject
            let parsed: Result<SearchSettingsByObject, XmlError> = SearchSettingsByObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssociationEngineSettings" => {
            // try to parse as AssociationEngineSettings
            let parsed: Result<AssociationEngineSettings, XmlError> = AssociationEngineSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotVariableOperation" => {
            // try to parse as BotVariableOperation
            let parsed: Result<BotVariableOperation, XmlError> = BotVariableOperation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EntitlementTemplate" => {
            // try to parse as EntitlementTemplate
            let parsed: Result<EntitlementTemplate, XmlError> = EntitlementTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SchedulingRule" => {
            // try to parse as SchedulingRule
            let parsed: Result<SchedulingRule, XmlError> = SchedulingRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformEventSubscriberConfig" => {
            // try to parse as PlatformEventSubscriberConfig
            let parsed: Result<PlatformEventSubscriberConfig, XmlError> = PlatformEventSubscriberConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktCalcInsightObjectDef" => {
            // try to parse as MktCalcInsightObjectDef
            let parsed: Result<MktCalcInsightObjectDef, XmlError> = MktCalcInsightObjectDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataSourceBundleDefinition" => {
            // try to parse as DataSourceBundleDefinition
            let parsed: Result<DataSourceBundleDefinition, XmlError> = DataSourceBundleDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MarketingAppExtension" => {
            // try to parse as MarketingAppExtension
            let parsed: Result<MarketingAppExtension, XmlError> = MarketingAppExtension::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextTag" => {
            // try to parse as ContextTag
            let parsed: Result<ContextTag, XmlError> = ContextTag::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPromptTemplateDataProvider" => {
            // try to parse as GenAiPromptTemplateDataProvider
            let parsed: Result<GenAiPromptTemplateDataProvider, XmlError> = GenAiPromptTemplateDataProvider::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DiscoveryCustomPrescribableFieldDefinition" => {
            // try to parse as DiscoveryCustomPrescribableFieldDefinition
            let parsed: Result<DiscoveryCustomPrescribableFieldDefinition, XmlError> = DiscoveryCustomPrescribableFieldDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityCustomThemeLayoutType" => {
            // try to parse as CommunityCustomThemeLayoutType
            let parsed: Result<CommunityCustomThemeLayoutType, XmlError> = CommunityCustomThemeLayoutType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataSource" => {
            // try to parse as DataSource
            let parsed: Result<DataSource, XmlError> = DataSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ObjectMappingField" => {
            // try to parse as ObjectMappingField
            let parsed: Result<ObjectMappingField, XmlError> = ObjectMappingField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ScoreRangeClassification" => {
            // try to parse as ScoreRangeClassification
            let parsed: Result<ScoreRangeClassification, XmlError> = ScoreRangeClassification::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CustomPageWebLink" => {
            // try to parse as CustomPageWebLink
            let parsed: Result<CustomPageWebLink, XmlError> = CustomPageWebLink::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordAggregationObjectFilter" => {
            // try to parse as RecordAggregationObjectFilter
            let parsed: Result<RecordAggregationObjectFilter, XmlError> = RecordAggregationObjectFilter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Tag" => {
            // try to parse as Tag
            let parsed: Result<Tag, XmlError> = Tag::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserLicenseDefinition" => {
            // try to parse as UserLicenseDefinition
            let parsed: Result<UserLicenseDefinition, XmlError> = UserLicenseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MktDataTranField" => {
            // try to parse as MktDataTranField
            let parsed: Result<MktDataTranField, XmlError> = MktDataTranField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniSupervisorConfigProfile" => {
            // try to parse as OmniSupervisorConfigProfile
            let parsed: Result<OmniSupervisorConfigProfile, XmlError> = OmniSupervisorConfigProfile::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LifeSciConfigAssignment" => {
            // try to parse as LifeSciConfigAssignment
            let parsed: Result<LifeSciConfigAssignment, XmlError> = LifeSciConfigAssignment::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssessmentQuestionVersion" => {
            // try to parse as AssessmentQuestionVersion
            let parsed: Result<AssessmentQuestionVersion, XmlError> = AssessmentQuestionVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalEntryCriteria" => {
            // try to parse as ApprovalEntryCriteria
            let parsed: Result<ApprovalEntryCriteria, XmlError> = ApprovalEntryCriteria::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CallCenter" => {
            // try to parse as CallCenter
            let parsed: Result<CallCenter, XmlError> = CallCenter::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CaseSettings" => {
            // try to parse as CaseSettings
            let parsed: Result<CaseSettings, XmlError> = CaseSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserProvisioningConfig" => {
            // try to parse as UserProvisioningConfig
            let parsed: Result<UserProvisioningConfig, XmlError> = UserProvisioningConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppCanvasStngs" => {
            // try to parse as ExtlClntAppCanvasStngs
            let parsed: Result<ExtlClntAppCanvasStngs, XmlError> = ExtlClntAppCanvasStngs::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AssistantSkillQuickAction" => {
            // try to parse as AssistantSkillQuickAction
            let parsed: Result<AssistantSkillQuickAction, XmlError> = AssistantSkillQuickAction::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WebStoreTemplate" => {
            // try to parse as WebStoreTemplate
            let parsed: Result<WebStoreTemplate, XmlError> = WebStoreTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvAcctFrcstDplyGroupItem" => {
            // try to parse as AdvAcctFrcstDplyGroupItem
            let parsed: Result<AdvAcctFrcstDplyGroupItem, XmlError> = AdvAcctFrcstDplyGroupItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CompactLayout" => {
            // try to parse as CompactLayout
            let parsed: Result<CompactLayout, XmlError> = CompactLayout::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BrandingSetProperty" => {
            // try to parse as BrandingSetProperty
            let parsed: Result<BrandingSetProperty, XmlError> = BrandingSetProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunityThemeSetting" => {
            // try to parse as CommunityThemeSetting
            let parsed: Result<CommunityThemeSetting, XmlError> = CommunityThemeSetting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SvcCatalogFulfillFlowItem" => {
            // try to parse as SvcCatalogFulfillFlowItem
            let parsed: Result<SvcCatalogFulfillFlowItem, XmlError> = SvcCatalogFulfillFlowItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AutoResponseRule" => {
            // try to parse as AutoResponseRule
            let parsed: Result<AutoResponseRule, XmlError> = AutoResponseRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeedFilterCriterion" => {
            // try to parse as FeedFilterCriterion
            let parsed: Result<FeedFilterCriterion, XmlError> = FeedFilterCriterion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardDynamicValue" => {
            // try to parse as DashboardDynamicValue
            let parsed: Result<DashboardDynamicValue, XmlError> = DashboardDynamicValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EinsteinDocumentCaptureSettings" => {
            // try to parse as EinsteinDocumentCaptureSettings
            let parsed: Result<EinsteinDocumentCaptureSettings, XmlError> = EinsteinDocumentCaptureSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotSettings" => {
            // try to parse as BotSettings
            let parsed: Result<BotSettings, XmlError> = BotSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UnnamedChildFTestMd2" => {
            // try to parse as UnnamedChildFTestMd2
            let parsed: Result<UnnamedChildFTestMd2, XmlError> = UnnamedChildFTestMd2::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SearchableObjDataSyncInfo" => {
            // try to parse as SearchableObjDataSyncInfo
            let parsed: Result<SearchableObjDataSyncInfo, XmlError> = SearchableObjDataSyncInfo::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SequenceServiceSettings" => {
            // try to parse as SequenceServiceSettings
            let parsed: Result<SequenceServiceSettings, XmlError> = SequenceServiceSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionParameters" => {
            // try to parse as QuickActionParameters
            let parsed: Result<QuickActionParameters, XmlError> = QuickActionParameters::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MlModelEndpoint" => {
            // try to parse as MlModelEndpoint
            let parsed: Result<MlModelEndpoint, XmlError> = MlModelEndpoint::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotNavigation" => {
            // try to parse as BotNavigation
            let parsed: Result<BotNavigation, XmlError> = BotNavigation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceProcessItemGroupTranslation" => {
            // try to parse as ServiceProcessItemGroupTranslation
            let parsed: Result<ServiceProcessItemGroupTranslation, XmlError> = ServiceProcessItemGroupTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PresenceConfigProfileAssignments" => {
            // try to parse as PresenceConfigProfileAssignments
            let parsed: Result<PresenceConfigProfileAssignments, XmlError> = PresenceConfigProfileAssignments::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardComponentGroupingSort" => {
            // try to parse as DashboardComponentGroupingSort
            let parsed: Result<DashboardComponentGroupingSort, XmlError> = DashboardComponentGroupingSort::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetTabSetting" => {
            // try to parse as PermissionSetTabSetting
            let parsed: Result<PermissionSetTabSetting, XmlError> = PermissionSetTabSetting::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TagOption" => {
            // try to parse as TagOption
            let parsed: Result<TagOption, XmlError> = TagOption::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppOauthIdToken" => {
            // try to parse as ConnectedAppOauthIdToken
            let parsed: Result<ConnectedAppOauthIdToken, XmlError> = ConnectedAppOauthIdToken::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramProcessRule" => {
            // try to parse as LoyaltyProgramProcessRule
            let parsed: Result<LoyaltyProgramProcessRule, XmlError> = LoyaltyProgramProcessRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataspaceScopeSchemaAccess" => {
            // try to parse as DataspaceScopeSchemaAccess
            let parsed: Result<DataspaceScopeSchemaAccess, XmlError> = DataspaceScopeSchemaAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IntegrationProviderDef" => {
            // try to parse as IntegrationProviderDef
            let parsed: Result<IntegrationProviderDef, XmlError> = IntegrationProviderDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ComponentInstancePropertyList" => {
            // try to parse as ComponentInstancePropertyList
            let parsed: Result<ComponentInstancePropertyList, XmlError> = ComponentInstancePropertyList::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowElementReferenceOrValue" => {
            // try to parse as FlowElementReferenceOrValue
            let parsed: Result<FlowElementReferenceOrValue, XmlError> = FlowElementReferenceOrValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesLsCommercialSettings" => {
            // try to parse as IndustriesLsCommercialSettings
            let parsed: Result<IndustriesLsCommercialSettings, XmlError> = IndustriesLsCommercialSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QuickActionParametersTranslation" => {
            // try to parse as QuickActionParametersTranslation
            let parsed: Result<QuickActionParametersTranslation, XmlError> = QuickActionParametersTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OauthTokenExchangeHandler" => {
            // try to parse as OauthTokenExchangeHandler
            let parsed: Result<OauthTokenExchangeHandler, XmlError> = OauthTokenExchangeHandler::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "NavigationMenuItem" => {
            // try to parse as NavigationMenuItem
            let parsed: Result<NavigationMenuItem, XmlError> = NavigationMenuItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ExtlClntAppOauthConfigurablePolicies" => {
            // try to parse as ExtlClntAppOauthConfigurablePolicies
            let parsed: Result<ExtlClntAppOauthConfigurablePolicies, XmlError> = ExtlClntAppOauthConfigurablePolicies::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SurveySettings" => {
            // try to parse as SurveySettings
            let parsed: Result<SurveySettings, XmlError> = SurveySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionLinkGroupTemplate" => {
            // try to parse as ActionLinkGroupTemplate
            let parsed: Result<ActionLinkGroupTemplate, XmlError> = ActionLinkGroupTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "TagProperty" => {
            // try to parse as TagProperty
            let parsed: Result<TagProperty, XmlError> = TagProperty::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ListMetadataQuery" => {
            // try to parse as ListMetadataQuery
            let parsed: Result<ListMetadataQuery, XmlError> = ListMetadataQuery::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MessagingAuthorization" => {
            // try to parse as MessagingAuthorization
            let parsed: Result<MessagingAuthorization, XmlError> = MessagingAuthorization::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IncludedUserLicenseDefinition" => {
            // try to parse as IncludedUserLicenseDefinition
            let parsed: Result<IncludedUserLicenseDefinition, XmlError> = IncludedUserLicenseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportLayoutSection" => {
            // try to parse as ReportLayoutSection
            let parsed: Result<ReportLayoutSection, XmlError> = ReportLayoutSection::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionPlanTemplateItem" => {
            // try to parse as ActionPlanTemplateItem
            let parsed: Result<ActionPlanTemplateItem, XmlError> = ActionPlanTemplateItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConnectedAppOauthConfig" => {
            // try to parse as ConnectedAppOauthConfig
            let parsed: Result<ConnectedAppOauthConfig, XmlError> = ConnectedAppOauthConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ManagedContentNodeType" => {
            // try to parse as ManagedContentNodeType
            let parsed: Result<ManagedContentNodeType, XmlError> = ManagedContentNodeType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InsRatePlanTypeConfig" => {
            // try to parse as InsRatePlanTypeConfig
            let parsed: Result<InsRatePlanTypeConfig, XmlError> = InsRatePlanTypeConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningTypeBundleResource" => {
            // try to parse as LightningTypeBundleResource
            let parsed: Result<LightningTypeBundleResource, XmlError> = LightningTypeBundleResource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CMSConnectSource" => {
            // try to parse as CMSConnectSource
            let parsed: Result<CMSConnectSource, XmlError> = CMSConnectSource::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIFilterGroup" => {
            // try to parse as AIFilterGroup
            let parsed: Result<AIFilterGroup, XmlError> = AIFilterGroup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventSubtype" => {
            // try to parse as EventSubtype
            let parsed: Result<EventSubtype, XmlError> = EventSubtype::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeWorkOrderLineItemField" => {
            // try to parse as KnowledgeWorkOrderLineItemField
            let parsed: Result<KnowledgeWorkOrderLineItemField, XmlError> = KnowledgeWorkOrderLineItemField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OcrTemplate" => {
            // try to parse as OcrTemplate
            let parsed: Result<OcrTemplate, XmlError> = OcrTemplate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPromptTemplateInput" => {
            // try to parse as GenAiPromptTemplateInput
            let parsed: Result<GenAiPromptTemplateInput, XmlError> = GenAiPromptTemplateInput::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetObjectPermissions" => {
            // try to parse as PermissionSetObjectPermissions
            let parsed: Result<PermissionSetObjectPermissions, XmlError> = PermissionSetObjectPermissions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ForecastingObjectListLabelMapping" => {
            // try to parse as ForecastingObjectListLabelMapping
            let parsed: Result<ForecastingObjectListLabelMapping, XmlError> = ForecastingObjectListLabelMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SceGlobalModelOptOutSettings" => {
            // try to parse as SceGlobalModelOptOutSettings
            let parsed: Result<SceGlobalModelOptOutSettings, XmlError> = SceGlobalModelOptOutSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FeatureParameterBoolean" => {
            // try to parse as FeatureParameterBoolean
            let parsed: Result<FeatureParameterBoolean, XmlError> = FeatureParameterBoolean::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MatchingRules" => {
            // try to parse as MatchingRules
            let parsed: Result<MatchingRules, XmlError> = MatchingRules::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "QueueMembers" => {
            // try to parse as QueueMembers
            let parsed: Result<QueueMembers, XmlError> = QueueMembers::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ModerationRule" => {
            // try to parse as ModerationRule
            let parsed: Result<ModerationRule, XmlError> = ModerationRule::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ApprovalStepRejectBehavior" => {
            // try to parse as ApprovalStepRejectBehavior
            let parsed: Result<ApprovalStepRejectBehavior, XmlError> = ApprovalStepRejectBehavior::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "GenAiPlannerBundle" => {
            // try to parse as GenAiPlannerBundle
            let parsed: Result<GenAiPlannerBundle, XmlError> = GenAiPlannerBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CareSystemFieldMapping" => {
            // try to parse as CareSystemFieldMapping
            let parsed: Result<CareSystemFieldMapping, XmlError> = CareSystemFieldMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetFieldPermissions" => {
            // try to parse as PermissionSetFieldPermissions
            let parsed: Result<PermissionSetFieldPermissions, XmlError> = PermissionSetFieldPermissions::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlowCategory" => {
            // try to parse as FlowCategory
            let parsed: Result<FlowCategory, XmlError> = FlowCategory::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIConvSummarizationConfig" => {
            // try to parse as AIConvSummarizationConfig
            let parsed: Result<AIConvSummarizationConfig, XmlError> = AIConvSummarizationConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AdvancedFieldMapping" => {
            // try to parse as AdvancedFieldMapping
            let parsed: Result<AdvancedFieldMapping, XmlError> = AdvancedFieldMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeCaseField" => {
            // try to parse as KnowledgeCaseField
            let parsed: Result<KnowledgeCaseField, XmlError> = KnowledgeCaseField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CancelDeployResult" => {
            // try to parse as CancelDeployResult
            let parsed: Result<CancelDeployResult, XmlError> = CancelDeployResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SchemaDefinition" => {
            // try to parse as SchemaDefinition
            let parsed: Result<SchemaDefinition, XmlError> = SchemaDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmbeddedServiceFlow" => {
            // try to parse as EmbeddedServiceFlow
            let parsed: Result<EmbeddedServiceFlow, XmlError> = EmbeddedServiceFlow::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InternalDataConnector" => {
            // try to parse as InternalDataConnector
            let parsed: Result<InternalDataConnector, XmlError> = InternalDataConnector::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DocumentCategoryDocumentType" => {
            // try to parse as DocumentCategoryDocumentType
            let parsed: Result<DocumentCategoryDocumentType, XmlError> = DocumentCategoryDocumentType::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CloudServiceProviderApi" => {
            // try to parse as CloudServiceProviderApi
            let parsed: Result<CloudServiceProviderApi, XmlError> = CloudServiceProviderApi::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsDashboardWidget" => {
            // try to parse as AnalyticsDashboardWidget
            let parsed: Result<AnalyticsDashboardWidget, XmlError> = AnalyticsDashboardWidget::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Approver" => {
            // try to parse as Approver
            let parsed: Result<Approver, XmlError> = Approver::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "WebStoreBundle" => {
            // try to parse as WebStoreBundle
            let parsed: Result<WebStoreBundle, XmlError> = WebStoreBundle::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "KnowledgeGenerationSettings" => {
            // try to parse as KnowledgeGenerationSettings
            let parsed: Result<KnowledgeGenerationSettings, XmlError> = KnowledgeGenerationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FlexiPageTemplateInstance" => {
            // try to parse as FlexiPageTemplateInstance
            let parsed: Result<FlexiPageTemplateInstance, XmlError> = FlexiPageTemplateInstance::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ServiceMgmtKnwlgArtclConfig" => {
            // try to parse as ServiceMgmtKnwlgArtclConfig
            let parsed: Result<ServiceMgmtKnwlgArtclConfig, XmlError> = ServiceMgmtKnwlgArtclConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChatterEmailsMDSettings" => {
            // try to parse as ChatterEmailsMDSettings
            let parsed: Result<ChatterEmailsMDSettings, XmlError> = ChatterEmailsMDSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "SiteWebAddress" => {
            // try to parse as SiteWebAddress
            let parsed: Result<SiteWebAddress, XmlError> = SiteWebAddress::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "Bot" => {
            // try to parse as Bot
            let parsed: Result<Bot, XmlError> = Bot::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ContextDefinitionVersion" => {
            // try to parse as ContextDefinitionVersion
            let parsed: Result<ContextDefinitionVersion, XmlError> = ContextDefinitionVersion::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ESignatureEnvelopeConfig" => {
            // try to parse as ESignatureEnvelopeConfig
            let parsed: Result<ESignatureEnvelopeConfig, XmlError> = ESignatureEnvelopeConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionPlanTemplateItemValue" => {
            // try to parse as ActionPlanTemplateItemValue
            let parsed: Result<ActionPlanTemplateItemValue, XmlError> = ActionPlanTemplateItemValue::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsVisualization" => {
            // try to parse as AnalyticsVisualization
            let parsed: Result<AnalyticsVisualization, XmlError> = AnalyticsVisualization::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "HomePageComponent" => {
            // try to parse as HomePageComponent
            let parsed: Result<HomePageComponent, XmlError> = HomePageComponent::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EventDelivery" => {
            // try to parse as EventDelivery
            let parsed: Result<EventDelivery, XmlError> = EventDelivery::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AnalyticsFilterWidgetDef" => {
            // try to parse as AnalyticsFilterWidgetDef
            let parsed: Result<AnalyticsFilterWidgetDef, XmlError> = AnalyticsFilterWidgetDef::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RetailExecutionSettings" => {
            // try to parse as RetailExecutionSettings
            let parsed: Result<RetailExecutionSettings, XmlError> = RetailExecutionSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LoyaltyProgramSetup" => {
            // try to parse as LoyaltyProgramSetup
            let parsed: Result<LoyaltyProgramSetup, XmlError> = LoyaltyProgramSetup::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CommunitiesSettings" => {
            // try to parse as CommunitiesSettings
            let parsed: Result<CommunitiesSettings, XmlError> = CommunitiesSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetCustomMetadataTypeAccess" => {
            // try to parse as PermissionSetCustomMetadataTypeAccess
            let parsed: Result<PermissionSetCustomMetadataTypeAccess, XmlError> = PermissionSetCustomMetadataTypeAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IframeWhiteListUrl" => {
            // try to parse as IframeWhiteListUrl
            let parsed: Result<IframeWhiteListUrl, XmlError> = IframeWhiteListUrl::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EinsteinCopilotSettings" => {
            // try to parse as EinsteinCopilotSettings
            let parsed: Result<EinsteinCopilotSettings, XmlError> = EinsteinCopilotSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DashboardFlexTableComponentProperties" => {
            // try to parse as DashboardFlexTableComponentProperties
            let parsed: Result<DashboardFlexTableComponentProperties, XmlError> = DashboardFlexTableComponentProperties::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AIUsecaseDefinition" => {
            // try to parse as AIUsecaseDefinition
            let parsed: Result<AIUsecaseDefinition, XmlError> = AIUsecaseDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BotMessageTranslation" => {
            // try to parse as BotMessageTranslation
            let parsed: Result<BotMessageTranslation, XmlError> = BotMessageTranslation::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesUnifiedPromotionsSettings" => {
            // try to parse as IndustriesUnifiedPromotionsSettings
            let parsed: Result<IndustriesUnifiedPromotionsSettings, XmlError> = IndustriesUnifiedPromotionsSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "StandardValueSet" => {
            // try to parse as StandardValueSet
            let parsed: Result<StandardValueSet, XmlError> = StandardValueSet::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "BillingSettings" => {
            // try to parse as BillingSettings
            let parsed: Result<BillingSettings, XmlError> = BillingSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportAggregate" => {
            // try to parse as ReportAggregate
            let parsed: Result<ReportAggregate, XmlError> = ReportAggregate::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LightningBoltFeatures" => {
            // try to parse as LightningBoltFeatures
            let parsed: Result<LightningBoltFeatures, XmlError> = LightningBoltFeatures::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InventoryReplenishmentSettings" => {
            // try to parse as InventoryReplenishmentSettings
            let parsed: Result<InventoryReplenishmentSettings, XmlError> = InventoryReplenishmentSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "MfgProgramTemplateItem" => {
            // try to parse as MfgProgramTemplateItem
            let parsed: Result<MfgProgramTemplateItem, XmlError> = MfgProgramTemplateItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "OmniDataTransformItem" => {
            // try to parse as OmniDataTransformItem
            let parsed: Result<OmniDataTransformItem, XmlError> = OmniDataTransformItem::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "EmailIntegrationSettings" => {
            // try to parse as EmailIntegrationSettings
            let parsed: Result<EmailIntegrationSettings, XmlError> = EmailIntegrationSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConsentBannerSettings" => {
            // try to parse as ConsentBannerSettings
            let parsed: Result<ConsentBannerSettings, XmlError> = ConsentBannerSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecordAggregationObject" => {
            // try to parse as RecordAggregationObject
            let parsed: Result<RecordAggregationObject, XmlError> = RecordAggregationObject::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "AsyncResult" => {
            // try to parse as AsyncResult
            let parsed: Result<AsyncResult, XmlError> = AsyncResult::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "InsPolicyManagementConfig" => {
            // try to parse as InsPolicyManagementConfig
            let parsed: Result<InsPolicyManagementConfig, XmlError> = InsPolicyManagementConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ActionableListDefinition" => {
            // try to parse as ActionableListDefinition
            let parsed: Result<ActionableListDefinition, XmlError> = ActionableListDefinition::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CampaignInfluenceModel" => {
            // try to parse as CampaignInfluenceModel
            let parsed: Result<CampaignInfluenceModel, XmlError> = CampaignInfluenceModel::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PermissionSetAgentAccess" => {
            // try to parse as PermissionSetAgentAccess
            let parsed: Result<PermissionSetAgentAccess, XmlError> = PermissionSetAgentAccess::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ReportHistoricalSelector" => {
            // try to parse as ReportHistoricalSelector
            let parsed: Result<ReportHistoricalSelector, XmlError> = ReportHistoricalSelector::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "UserManagementSettings" => {
            // try to parse as UserManagementSettings
            let parsed: Result<UserManagementSettings, XmlError> = UserManagementSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "DataSourceField" => {
            // try to parse as DataSourceField
            let parsed: Result<DataSourceField, XmlError> = DataSourceField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RecommendationBuilderSettings" => {
            // try to parse as RecommendationBuilderSettings
            let parsed: Result<RecommendationBuilderSettings, XmlError> = RecommendationBuilderSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "FieldMapping" => {
            // try to parse as FieldMapping
            let parsed: Result<FieldMapping, XmlError> = FieldMapping::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "CountriesAndStates" => {
            // try to parse as CountriesAndStates
            let parsed: Result<CountriesAndStates, XmlError> = CountriesAndStates::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "PlatformEventSettings" => {
            // try to parse as PlatformEventSettings
            let parsed: Result<PlatformEventSettings, XmlError> = PlatformEventSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "LiveChatAgentConfig" => {
            // try to parse as LiveChatAgentConfig
            let parsed: Result<LiveChatAgentConfig, XmlError> = LiveChatAgentConfig::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "IndustriesLoyaltySettings" => {
            // try to parse as IndustriesLoyaltySettings
            let parsed: Result<IndustriesLoyaltySettings, XmlError> = IndustriesLoyaltySettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ChannelRevMgmtSettings" => {
            // try to parse as ChannelRevMgmtSettings
            let parsed: Result<ChannelRevMgmtSettings, XmlError> = ChannelRevMgmtSettings::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "ConversationMessageMergeField" => {
            // try to parse as ConversationMessageMergeField
            let parsed: Result<ConversationMessageMergeField, XmlError> = ConversationMessageMergeField::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        "RetrieveRequest" => {
            // try to parse as RetrieveRequest
            let parsed: Result<RetrieveRequest, XmlError> = RetrieveRequest::from_metadata_xml(xml);
            match parsed {
                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),
                Err(e) => return Err(e),
            }
        }
        _ => Err(XmlError::Deserialize(format!("Unknown root: {}", root))),
    }
}
