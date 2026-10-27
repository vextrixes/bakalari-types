use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::SchoolClass;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    #[serde(rename = "UserUID")]
    pub user_uid: String,
    pub campaign_category_code: String,
    pub class: SchoolClass, //check nullable
    pub teacher_classes: Option<Vec<SchoolClass>>,
    pub full_name: String, //check nullable
    pub school_organization_name: Option<String>,
    pub school_type: Option<String>,
    pub user_type: String, //Enum "controller", "parents", "student", "teacher"
    pub user_type_text: String,
    pub study_year: Option<i64>,
    pub enabled_modules: Vec<Module>,
    pub setting_modules: HashMap<String, Value>, // TODO: Setting Module
    pub full_user_name: Option<String>,
    pub students: Vec<UserInfoStudent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub module: String, //Make an Enum "absence", "accesssystem", "application", "campaign", "classbook", "emergency", "events", "gdpr", "homeworks", "komens", "marking", "marks", "payments", "semester", "subjects", "substitutions", "teaching", "timetable", "unknown", "user_account", "web_based"
    pub rights: Vec<String>, //Make enum "CanAccessEmergency",  "CanBeEmergencyParticipant",  "CanConfirmClassification",  "CanExcuseAbsence",  "CanManageEmergencies",  "CanOverseeEmergencyParticipants",  "ClassFundEnabled",  "ClassFundPaymentsEnabled",  "EditClassBook",  "EditMarks",  "PredictMarks",  "RegularPaymentsEnabled",  "SendAttachments",  "SendMessageAsDirector",  "SendMessages",  "ShowAbsence",  "ShowAbsencePercentage",  "ShowApologyMessages",  "ShowCampaign",  "ShowChildConsents",  "ShowClassBook",  "ShowClassFundDetails",  "ShowCommissioners",  "ShowEvents",  "ShowFinalMarks",  "ShowHomeworks",  "ShowManners",  "ShowMarking",  "ShowMarks",  "ShowNoticeBoardMessages",  "ShowOwnConsents",  "ShowRatingDetails",  "ShowRatingMessage",  "ShowReceivedMessages",  "ShowSentMessages",  "ShowSubjectThemes",  "ShowSubjects",  "ShowSubstitutions",  "ShowTimetable",  "Unknown"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfoStudent {
    pub full_name: String,
    pub class: Vec<SchoolClass>,
    pub school_type: String, //Enum "BasicArtSchool","BasicPracticalSchool","BasicSchool","BasicSpecialSchool","BoardingHouse","ChildrensHome","Club","Conservatory","EconomicSection","Gymnasium","HigerProfessionalSchool","Kindergarten","OtherSchool","OtherType","PracticalSchool","SchoolCanteen","SchoolClub","SchoolService","SecondarySchool","SecondaryVocationalSchool","SportGymnasium","SuperiorOrganization","TechnicalSchool","University","Unknown","VocationalSchool"
    pub study_year: Option<i64>,
}
