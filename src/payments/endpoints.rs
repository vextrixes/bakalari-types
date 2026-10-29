use std::marker::PhantomData;

use crate::{
    ClassFundMonthly, ClassFundPaymentInfo, ClassFundSummary, ClassFundTeacherDocumentDetail,
    ClassFundTeacherDocumentsMonthly, ClassFundTeacherStudentsItem, ClassFundTeacherSummary,
    Endpoint, RegularPayment, RegularPaymentDetail, RegularPaymentsSummary, RegularPrescription,
    RegularPrescriptionDetail, RegularPrescriptionStudent, RegularPrescriptionStudentDetail,
};

pub const CLASS_FUND_PAYMENT_INFO_ENDPOINT: Endpoint<(), ClassFundPaymentInfo> = Endpoint::Get {
    url: "/api/3/payments/classfund/paymentsinfo",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PAYMENTS_INFO_ENDPOINT: Endpoint<(), Vec<RegularPayment>> = Endpoint::Get {
    url: "/api/3/payments/regularpayments",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PAYMENTS_SUMMARY_ENDPOINT: Endpoint<(), RegularPaymentsSummary> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/summary",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_INFO_ENDPOINT: Endpoint<(), Vec<ClassFundMonthly>> = Endpoint::Get {
    url: "/api/3/payments/classfund",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_SUMMARY_ENDPOINT: Endpoint<(), ClassFundSummary> = Endpoint::Get {
    url: "/api/3/payments/classfund/summary",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PAYMENT_DETAIL_ENDPOINT: Endpoint<(), RegularPaymentDetail> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/detail/{paymentId}",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_TEACHER_DOCUMENT_DETAIL_ENDPOINT: Endpoint<
    (),
    ClassFundTeacherDocumentDetail,
> = Endpoint::Get {
    url: "/api/3/payments/classfund/teacher/document/detail/{documentId}",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PRESCRIPTIONS_ENDPOINT: Endpoint<(), Vec<RegularPrescription>> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/teacher/prescriptions",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_TEACHER_STUDENTS_ENDPOINT: Endpoint<(), Vec<ClassFundTeacherStudentsItem>> =
    Endpoint::Get {
        url: "/api/3/payments/classfund/teacher/students/{classId}",
        auth: true,
        _types: PhantomData,
    };

pub const CLASS_FUND_TEACHER_DOCUMENTS_ENDPOINT: Endpoint<
    (),
    Vec<ClassFundTeacherDocumentsMonthly>,
> = Endpoint::Get {
    url: "/api/3/payments/classfund/teacher/documents/{classId}",
    auth: true,
    _types: PhantomData,
};

pub const CLASS_FUND_TEACHER_SUMMARY_ENDPOINT: Endpoint<(), ClassFundTeacherSummary> =
    Endpoint::Get {
        url: "/api/3/payments/classfund/teacher/summary",
        auth: true,
        _types: PhantomData,
    };

pub const REGULAR_PRESCRIPTION_DETAIL_ENDPOINT: Endpoint<(), RegularPrescriptionDetail> =
    Endpoint::Get {
        url: "/api/3/payments/regularpayments/teacher/prescriptions/{prescriptionId}",
        auth: true,
        _types: PhantomData,
    };

pub const REGULAR_PRESCRIPTION_STUDENT_DETAIL_ENDPOINT: Endpoint<
    (),
    RegularPrescriptionStudentDetail,
> = Endpoint::Get {
    url: "/api/3/payments/regularpayments/teacher/prescriptions/{prescriptionId}/students/{studentId}",
    auth: true,
    _types: PhantomData,
};

pub const REGULAR_PRESCRIPTION_STUDENTS_ENDPOINT: Endpoint<(), Vec<RegularPrescriptionStudent>> =
    Endpoint::Get {
        url: "/api/3/payments/regularpayments/teacher/prescriptions/{prescriptionId}/students",
        auth: true,
        _types: PhantomData,
    };
