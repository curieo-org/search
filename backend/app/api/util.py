from enum import Enum


class RouteCategory(str, Enum):
    ClinicalTrials = "clinicaltrials"
    DRUG = "drug"
    PUBMED_BIOXRIV_WEB = "pubmed_bioxriv_web"
    NOT_SELECTED = "not_selected"
