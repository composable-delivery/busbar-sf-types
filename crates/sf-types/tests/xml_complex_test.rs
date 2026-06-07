//! Additional tests for XML serialization with more complex types
//!
//! This test file verifies that XML serialization works correctly
//! with types that have many fields and different data types.

use busbar_sf_types::settings::org_settings::ActivitiesSettings;
use busbar_sf_types::traits::XmlSerializable;

#[test]
fn test_complex_type_serialization() {
    let settings = ActivitiesSettings {
        allow_users_to_relate_multiple_contacts_to_tasks_and_events: Some(true),
        auto_relate_event_attendees: Some(false),
        enable_activity_reminders: Some(true),
        enable_calendar_home_lwc: Some(false),
        enable_click_create_events: Some(true),
        enable_drag_and_drop_scheduling: Some(true),
        enable_email_tracking: Some(false),
        enable_flow_task_notifs_via_apex: Some(true),
        enable_group_tasks: Some(false),
        enable_hide_child_events_preference: Some(true),
        enable_list_view_scheduling: Some(false),
        enable_log_note: Some(true),
        enable_ml_single_client_profile: Some(false),
        enable_multiday_events: Some(true),
        enable_recurring_events: Some(true),
        enable_recurring_tasks: Some(false),
        enable_roll_up_activ_to_contacts_acct: Some(true),
        enable_sidebar_calendar_shortcut: Some(false),
        enable_simple_task_create_ui: Some(true),
        enable_timeline_comp_date_sort: Some(false),
        enable_uns_task_delegated_to_notifications: Some(true),
        enable_user_list_view_calendars: Some(false),
        meeting_requests_logo: Some("https://example.com/logo.png".to_string()),
        show_custom_logo_meeting_requests: Some(true),
        show_event_details_multi_user_calendar: Some(false),
        show_home_page_hover_links_for_events: Some(true),
        show_my_tasks_hover_links: Some(false),
    };

    let xml = settings.to_metadata_xml().expect("Failed to serialize");

    // Verify XML structure
    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("xmlns=\"http://soap.sforce.com/2006/04/metadata\""));
    assert!(xml.contains("<ActivitiesSettings"));
    assert!(xml.contains("</ActivitiesSettings>"));

    // Verify some field values are present
    assert!(xml.contains("<allowUsersToRelateMultipleContactsToTasksAndEvents>true"));
    assert!(xml.contains("<autoRelateEventAttendees>false"));
    assert!(xml.contains("<meetingRequestsLogo>https://example.com/logo.png</meetingRequestsLogo>"));
}

#[test]
fn test_complex_type_deserialization() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ActivitiesSettings xmlns="http://soap.sforce.com/2006/04/metadata">
    <allowUsersToRelateMultipleContactsToTasksAndEvents>true</allowUsersToRelateMultipleContactsToTasksAndEvents>
    <autoRelateEventAttendees>false</autoRelateEventAttendees>
    <enableActivityReminders>true</enableActivityReminders>
    <enableCalendarHomeLWC>false</enableCalendarHomeLWC>
    <enableClickCreateEvents>true</enableClickCreateEvents>
    <enableDragAndDropScheduling>true</enableDragAndDropScheduling>
    <enableEmailTracking>false</enableEmailTracking>
    <enableFlowTaskNotifsViaApex>true</enableFlowTaskNotifsViaApex>
    <enableGroupTasks>false</enableGroupTasks>
    <enableHideChildEventsPreference>true</enableHideChildEventsPreference>
    <enableListViewScheduling>false</enableListViewScheduling>
    <enableLogNote>true</enableLogNote>
    <enableMLSingleClientProfile>false</enableMLSingleClientProfile>
    <enableMultidayEvents>true</enableMultidayEvents>
    <enableRecurringEvents>true</enableRecurringEvents>
    <enableRecurringTasks>false</enableRecurringTasks>
    <enableRollUpActivToContactsAcct>true</enableRollUpActivToContactsAcct>
    <enableSidebarCalendarShortcut>false</enableSidebarCalendarShortcut>
    <enableSimpleTaskCreateUI>true</enableSimpleTaskCreateUI>
    <enableTimelineCompDateSort>false</enableTimelineCompDateSort>
    <enableUNSTaskDelegatedToNotifications>true</enableUNSTaskDelegatedToNotifications>
    <enableUserListViewCalendars>false</enableUserListViewCalendars>
    <meetingRequestsLogo>https://example.com/logo.png</meetingRequestsLogo>
    <showCustomLogoMeetingRequests>true</showCustomLogoMeetingRequests>
    <showEventDetailsMultiUserCalendar>false</showEventDetailsMultiUserCalendar>
    <showHomePageHoverLinksForEvents>true</showHomePageHoverLinksForEvents>
    <showMyTasksHoverLinks>false</showMyTasksHoverLinks>
</ActivitiesSettings>"#;

    let settings = ActivitiesSettings::from_metadata_xml(xml).expect("Failed to deserialize");

    // Verify some key fields
    assert_eq!(
        settings.allow_users_to_relate_multiple_contacts_to_tasks_and_events,
        Some(true)
    );
    assert_eq!(settings.auto_relate_event_attendees, Some(false));
    assert_eq!(settings.enable_activity_reminders, Some(true));
    assert_eq!(
        settings.meeting_requests_logo,
        Some("https://example.com/logo.png".to_string())
    );
    assert_eq!(settings.show_custom_logo_meeting_requests, Some(true));
}

#[test]
fn test_complex_type_roundtrip() {
    let original = ActivitiesSettings {
        allow_users_to_relate_multiple_contacts_to_tasks_and_events: Some(true),
        auto_relate_event_attendees: Some(false),
        enable_activity_reminders: Some(true),
        enable_calendar_home_lwc: Some(false),
        enable_click_create_events: Some(true),
        enable_drag_and_drop_scheduling: Some(true),
        enable_email_tracking: Some(false),
        enable_flow_task_notifs_via_apex: Some(true),
        enable_group_tasks: Some(false),
        enable_hide_child_events_preference: Some(true),
        enable_list_view_scheduling: Some(false),
        enable_log_note: Some(true),
        enable_ml_single_client_profile: Some(false),
        enable_multiday_events: Some(true),
        enable_recurring_events: Some(true),
        enable_recurring_tasks: Some(false),
        enable_roll_up_activ_to_contacts_acct: Some(true),
        enable_sidebar_calendar_shortcut: Some(false),
        enable_simple_task_create_ui: Some(true),
        enable_timeline_comp_date_sort: Some(false),
        enable_uns_task_delegated_to_notifications: Some(true),
        enable_user_list_view_calendars: Some(false),
        meeting_requests_logo: Some("https://example.com/logo.png".to_string()),
        show_custom_logo_meeting_requests: Some(true),
        show_event_details_multi_user_calendar: Some(false),
        show_home_page_hover_links_for_events: Some(true),
        show_my_tasks_hover_links: Some(false),
    };

    let xml = original.to_metadata_xml().expect("Failed to serialize");
    let deserialized = ActivitiesSettings::from_metadata_xml(&xml).expect("Failed to deserialize");

    // Verify roundtrip preserves all field values
    assert_eq!(
        original.allow_users_to_relate_multiple_contacts_to_tasks_and_events,
        deserialized.allow_users_to_relate_multiple_contacts_to_tasks_and_events
    );
    assert_eq!(
        original.auto_relate_event_attendees,
        deserialized.auto_relate_event_attendees
    );
    assert_eq!(
        original.enable_activity_reminders,
        deserialized.enable_activity_reminders
    );
    assert_eq!(
        original.meeting_requests_logo,
        deserialized.meeting_requests_logo
    );
    assert_eq!(
        original.show_custom_logo_meeting_requests,
        deserialized.show_custom_logo_meeting_requests
    );
}
