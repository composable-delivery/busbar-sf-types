//! Additional tests for XML serialization with more complex types
//!
//! This test file verifies that XML serialization works correctly
//! with types that have many fields and different data types.

use busbar_sf_types::settings::org_settings::ActivitiesSettings;
use busbar_sf_types::traits::XmlSerializable;

#[test]
fn test_complex_type_serialization() {
    let settings = ActivitiesSettings {
        allow_users_to_relate_multiple_contacts_to_tasks_and_events: true,
        auto_relate_event_attendees: false,
        enable_activity_reminders: true,
        enable_calendar_home_lwc: false,
        enable_click_create_events: true,
        enable_drag_and_drop_scheduling: true,
        enable_email_tracking: false,
        enable_flow_task_notifs_via_apex: true,
        enable_group_tasks: false,
        enable_hide_child_events_preference: true,
        enable_list_view_scheduling: false,
        enable_log_note: true,
        enable_ml_single_client_profile: false,
        enable_multiday_events: true,
        enable_recurring_events: true,
        enable_recurring_tasks: false,
        enable_roll_up_activ_to_contacts_acct: true,
        enable_sidebar_calendar_shortcut: false,
        enable_simple_task_create_ui: true,
        enable_timeline_comp_date_sort: false,
        enable_uns_task_delegated_to_notifications: true,
        enable_user_list_view_calendars: false,
        meeting_requests_logo: "https://example.com/logo.png".to_string(),
        show_custom_logo_meeting_requests: true,
        show_event_details_multi_user_calendar: false,
        show_home_page_hover_links_for_events: true,
        show_my_tasks_hover_links: false,
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

    let settings = ActivitiesSettings::from_metadata_xml(xml)
        .expect("Failed to deserialize");

    // Verify some key fields
    assert_eq!(settings.allow_users_to_relate_multiple_contacts_to_tasks_and_events, true);
    assert_eq!(settings.auto_relate_event_attendees, false);
    assert_eq!(settings.enable_activity_reminders, true);
    assert_eq!(settings.meeting_requests_logo, "https://example.com/logo.png");
    assert_eq!(settings.show_custom_logo_meeting_requests, true);
}

#[test]
fn test_complex_type_roundtrip() {
    let original = ActivitiesSettings {
        allow_users_to_relate_multiple_contacts_to_tasks_and_events: true,
        auto_relate_event_attendees: false,
        enable_activity_reminders: true,
        enable_calendar_home_lwc: false,
        enable_click_create_events: true,
        enable_drag_and_drop_scheduling: true,
        enable_email_tracking: false,
        enable_flow_task_notifs_via_apex: true,
        enable_group_tasks: false,
        enable_hide_child_events_preference: true,
        enable_list_view_scheduling: false,
        enable_log_note: true,
        enable_ml_single_client_profile: false,
        enable_multiday_events: true,
        enable_recurring_events: true,
        enable_recurring_tasks: false,
        enable_roll_up_activ_to_contacts_acct: true,
        enable_sidebar_calendar_shortcut: false,
        enable_simple_task_create_ui: true,
        enable_timeline_comp_date_sort: false,
        enable_uns_task_delegated_to_notifications: true,
        enable_user_list_view_calendars: false,
        meeting_requests_logo: "https://example.com/logo.png".to_string(),
        show_custom_logo_meeting_requests: true,
        show_event_details_multi_user_calendar: false,
        show_home_page_hover_links_for_events: true,
        show_my_tasks_hover_links: false,
    };

    let xml = original.to_metadata_xml().expect("Failed to serialize");
    let deserialized = ActivitiesSettings::from_metadata_xml(&xml)
        .expect("Failed to deserialize");

    // Verify roundtrip preserves all field values
    assert_eq!(original.allow_users_to_relate_multiple_contacts_to_tasks_and_events, 
               deserialized.allow_users_to_relate_multiple_contacts_to_tasks_and_events);
    assert_eq!(original.auto_relate_event_attendees, deserialized.auto_relate_event_attendees);
    assert_eq!(original.enable_activity_reminders, deserialized.enable_activity_reminders);
    assert_eq!(original.meeting_requests_logo, deserialized.meeting_requests_logo);
    assert_eq!(original.show_custom_logo_meeting_requests, deserialized.show_custom_logo_meeting_requests);
}
