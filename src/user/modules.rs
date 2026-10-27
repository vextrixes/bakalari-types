use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Module")]
pub enum Module {
    Absence { rights: Vec<AbsenceRights> },
    AccessSystem { rights: Vec<AccessSystemRights> },
    Application { rights: Vec<ApplicationRights> },
    Campaign { rights: Vec<CampaignRights> },
    Classbook { rights: Vec<ClassbookRights> },
    Emergency { rights: Vec<EmergencyRights> },
    Events { rights: Vec<EventsRights> },
    Gdpr { rights: Vec<GdprRights> },
    Homeworks { rights: Vec<HomeworksRights> },
    Komens { rights: Vec<KomensRights> },
    Marking { rights: Vec<MarkingRights> },
    Marks { rights: Vec<MarksRights> },
    Payments { rights: Vec<PaymentsRights> },
    Semester { rights: Vec<SemesterRights> },
    Subjects { rights: Vec<SubjectsRights> },
    Substitutions { rights: Vec<SubstitutionsRights> },
    Teaching { rights: Vec<TeachingRights> },
    Timetable { rights: Vec<TimetableRights> },
    Unknown { rights: Vec<UnknownRights> },
    UserAccount { rights: Vec<UserAccountRights> },
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
