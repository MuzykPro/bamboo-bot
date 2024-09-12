pub mod get_timesheet_entries {
    pub mod project;
    pub mod project_info;
    pub mod timesheet_entry;
    pub mod task;
}

pub mod add_timesheet_entry {
    pub mod add_entry_request;
}

pub mod get_whos_out {
    pub mod time_off_entry;
}

pub mod get_timeoff_requests {
    pub mod timeoff_request;
    pub mod actions;
    pub mod amount;
    pub mod notes;
    pub mod request_type;
    pub mod status;
}