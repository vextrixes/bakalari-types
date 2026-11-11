use crate::{
    endpoints::Endpoint,
    payments::classfund::{
        Monthly, PaymentInfo, Summary, TeacherDocumentDetail, TeacherDocumentsMonthly,
        TeacherStudentsItem, TeacherSummary,
    },
    payments::regularpayments::{
        Payment, PaymentDetail, PaymentsSummary, Prescription,
        PrescriptionDetail, PrescriptionStudent, PrescriptionStudentDetail,
    },
};
use std::marker::PhantomData;

pub const CLASS_FUND_PAYMENT_INFO_ENDPOINT: Endpoint<PaymentInfo, ()> = Endpoint::Get {
    url: "/api/3/payments/classfund/paymentsinfo",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PAYMENTS_INFO_ENDPOINT: Endpoint<Vec<Payment>, ()> = Endpoint::Get {
    url: "/api/3/payments/regularpayments",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PAYMENTS_SUMMARY_ENDPOINT: Endpoint<PaymentsSummary, ()> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/summary",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_INFO_ENDPOINT: Endpoint<Vec<Monthly>, ()> = Endpoint::Get {
    url: "/api/3/payments/classfund",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_SUMMARY_ENDPOINT: Endpoint<Summary, ()> = Endpoint::Get {
    url: "/api/3/payments/classfund/summary",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PAYMENT_DETAIL_ENDPOINT: Endpoint<PaymentDetail, ()> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/detail/{paymentId}",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_TEACHER_DOCUMENT_DETAIL_ENDPOINT: Endpoint<TeacherDocumentDetail, ()> =
    Endpoint::Get {
        url: "/api/3/payments/classfund/teacher/document/detail/{documentId}",
        auth: true,
        _types: PhantomData,
    };

pub const REGULAR_PRESCRIPTIONS_ENDPOINT: Endpoint<Vec<Prescription>, ()> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/teacher/prescriptions",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_TEACHER_STUDENTS_ENDPOINT: Endpoint<Vec<TeacherStudentsItem>, ()> =
    Endpoint::Get {
        url: "/api/3/payments/classfund/teacher/students/{classId}",
        auth: true,
        _types: PhantomData,
    };

pub const CLASS_FUND_TEACHER_DOCUMENTS_ENDPOINT: Endpoint<Vec<TeacherDocumentsMonthly>, ()> =
    Endpoint::Get {
        url: "/api/3/payments/classfund/teacher/documents/{classId}",
        auth: true,
        _types: PhantomData,
    };

pub const CLASS_FUND_TEACHER_SUMMARY_ENDPOINT: Endpoint<TeacherSummary, ()> = Endpoint::Get {
    url: "/api/3/payments/classfund/teacher/summary",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PRESCRIPTION_DETAIL_ENDPOINT: Endpoint<PrescriptionDetail, ()> =
    Endpoint::Get {
        url: "/api/3/payments/regularpayments/teacher/prescriptions/{prescriptionId}",
        auth: true,
        _types: PhantomData,
    };

pub const REGULAR_PRESCRIPTION_STUDENT_DETAIL_ENDPOINT: Endpoint<
    PrescriptionStudentDetail,
    (),
> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/teacher/prescriptions/{prescriptionId}/students/{studentId}",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PRESCRIPTION_STUDENTS_ENDPOINT: Endpoint<Vec<PrescriptionStudent>, ()> =
    Endpoint::Get {
        url: "/api/3/payments/regularpayments/teacher/prescriptions/{prescriptionId}/students",
        auth: true,
        _types: PhantomData,
    };
