use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Module")]
pub enum Module {
    #[serde(rename_all = "PascalCase")]
    Absence { rights: Vec<AbsenceRights> },
    #[serde(rename_all = "PascalCase")]
    AccessSystem { rights: Vec<AccessSystemRights> },
    #[serde(rename_all = "PascalCase")]
    Application { rights: Vec<ApplicationRights> },
    #[serde(rename_all = "PascalCase")]
    Campaign { rights: Vec<CampaignRights> },
    #[serde(rename_all = "PascalCase")]
    Classbook { rights: Vec<ClassbookRights> },
    #[serde(rename_all = "PascalCase")]
    Emergency { rights: Vec<EmergencyRights> },
    #[serde(rename_all = "PascalCase")]
    Events { rights: Vec<EventsRights> },
    #[serde(rename_all = "PascalCase")]
    Gdpr { rights: Vec<GdprRights> },
    #[serde(rename_all = "PascalCase")]
    Homeworks { rights: Vec<HomeworksRights> },
    #[serde(rename_all = "PascalCase")]
    Komens { rights: Vec<KomensRights> },
    #[serde(rename_all = "PascalCase")]
    Marking { rights: Vec<MarkingRights> },
    #[serde(rename_all = "PascalCase")]
    Marks { rights: Vec<MarksRights> },
    #[serde(rename_all = "PascalCase")]
    Payments { rights: Vec<PaymentsRights> },
    #[serde(rename_all = "PascalCase")]
    Semester { rights: Vec<SemesterRights> },
    #[serde(rename_all = "PascalCase")]
    Subjects { rights: Vec<SubjectsRights> },
    #[serde(rename_all = "PascalCase")]
    Substitutions { rights: Vec<SubstitutionsRights> },
    #[serde(rename_all = "PascalCase")]
    Teaching { rights: Vec<TeachingRights> },
    #[serde(rename_all = "PascalCase")]
    Timetable { rights: Vec<TimetableRights> },
    #[serde(rename_all = "PascalCase")]
    Unknown { rights: Vec<UnknownRights> },
    #[serde(rename_all = "PascalCase")]
    UserAccount { rights: Vec<UserAccountRights> },
    #[serde(rename_all = "PascalCase")]
    WebBased { rights: Vec<WebBasedRights> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbsenceRights {
    ShowAbsence,
    ShowAbsencePercentage,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessSystemRights {
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationRights {
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CampaignRights {
    ShowCampaign,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassbookRights {
    CanExcuseAbsence,
    EditClassBook,
    ShowClassBook,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyRights {
    CanAccessEmergency,
    CanBeEmergencyParticipant,
    CanManageEmergencies,
    CanOverseeEmergencyParticipants,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventsRights {
    ShowEvents,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GdprRights {
    ShowChildConsents,
    ShowCommissioners,
    ShowOwnConsents,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HomeworksRights {
    ShowHomeworks,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KomensRights {
    SendAttachments,
    SendMessageAsDirector,
    SendMessages,
    ShowApologyMessages,
    ShowNoticeBoardMessages,
    ShowRatingDetails,
    ShowRatingMessage,
    ShowReceivedMessages,
    ShowSentMessages,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkingRights {
    EditMarks,
    ShowMarking,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarksRights {
    CanConfirmClassification,
    PredictMarks,
    ShowFinalMarks,
    ShowManners,
    ShowMarks,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentsRights {
    ClassFundEnabled,
    ClassFundPaymentsEnabled,
    RegularPaymentsEnabled,
    ShowClassFundDetails,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SemesterRights {
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectsRights {
    ShowSubjectThemes,
    ShowSubjects,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstitutionsRights {
    ShowSubstitutions,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeachingRights {
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimetableRights {
    ShowTimetable,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnknownRights {
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserAccountRights {
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebBasedRights {
    #[serde(other)]
    Unknown,
}
