from enum import Enum

class RouteCategory(str, Enum):
    CT = "clinicaltrials"
    DRUG = "drug"
    PBW = "pubmed_bioxriv_web"
    NS = "not_selected"