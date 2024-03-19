# %%
# clinical_data_extraction.py
{
    "QUESTION": " Where did the study Safety and Tolerability of RNS60 Given by IV to Healthy Subjects take place ,anded",
    "CONTEXT": """Table 'tbl_studies_eligibilities' has columns: id (VARCHAR), title (TEXT), description (TEXT), eligibility_details (JSONB), and foreign keys: . The table description is: The table contains Information about the criteria used to select participants; includes inclusion and exclusion criteria. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'eligibility_details' has information in JSON format. The first field is 'Population' (field name is 'Population) which contains the information about the associated population type. The second field is the Sampling Method (field name is 'SamplingMethod') which contains the data about the method of sampling. The third field is Minimum Age (field name is 'MinimumAge') which contains data about the minimum age of the population. The fourth field is Maximum Age (field name is 'MaximumAge') which contains data about the maximum age of the population. Next field is 'HealthyVolunteers' that have information about the requirements of healthy volunteers. The last field is 'Criteria' which has the inclusion and exclusion criteria for the trial.
Table 'tbl_studies_adverse_details' has columns: id (VARCHAR), title (TEXT), description (TEXT), adverse_details (JSONB), and foreign keys: . The table description is: The table contains Summary information about reported adverse events (any untoward or unfavorable medical occurrence to participants, including abnormal physical exams, laboratory findings, symptoms, or diseases), including serious adverse events, other adverse events, and mortality. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.' title' is the title of the document.'description' is the description of the document. 'adverse_details' has information in JSON format. The first field is 'Event Type' (field name is 'EventType') which contains information about the type of adverse events like 'deaths' or 'serious'. The second field is about the count of subjects that are affected by the trial(field name is 'SubjestAffected'). The third field is 'Classification' which tells a detailed description of the Adverse Events.  The fourth field is about the count of subjects that are risked by the trial(field name is 'SubjectsRisk').
Table 'tbl_studies_info' has columns: nct_id (VARCHAR), title (TEXT), description (TEXT), study_details (JSON), and foreign keys: . The table description is: Summary of various clinical studies including topics such as HIV, bone marrow collection, knee replacement, Cancer, and many healthcare problems.The table has four columns: nct_id (VARCHAR), title (TEXT), description (TEXT), and study_details (JSON). Every row displays a document.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'study_details' has information about the source of the document (field name is 'source') and the type of the study like Observational or Interventional(field name is 'study_type'), Enrollment details (field name is 'enrollment'), Status of the clinical trial(field name is 'status'), phase of the clinical trial(field name is 'phase'), and important dates(field name is 'date') like start date and update date. The information is in the json format""",
    "OUTPUT": """SELECT tbl_studies_info.title, tbl_studies_info.study_details->>'source' AS study_source
FROM tbl_studies_info
WHERE tbl_studies_info.title = 'Safety and Tolerability of RNS60 Given by IV to Healthy Subjects';
SQLResult: 
| title                                                     | study_source |
|-----------------------------------------------------------|--------------|
| Safety and Tolerability of RNS60 Given by IV to Healthy Subjects | ClinicalTrials.gov |
Answer: The study "Safety and Tolerability of RNS60 Given by IV to Healthy Subjects" took place at ClinicalTrials.gov.""",
},
{
    "QUESTION": " How many participants was enrolled in Recipient Vessels for Free Tissue Flaps in Advanced Oncologic Defects of the Midface and Scalp,anded",
    "CONTEXT": """Table 'tbl_studies_eligibilities' has columns: id (VARCHAR), title (TEXT), description (TEXT), eligibility_details (JSONB), and foreign keys: . The table description is: The table contains Information about the criteria used to select participants; includes inclusion and exclusion criteria. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'eligibility_details' has information in JSON format. The first field is 'Population' (field name is 'Population) which contains the information about the associated population type. The second field is the Sampling Method (field name is 'SamplingMethod') which contains the data about the method of sampling. The third field is Minimum Age (field name is 'MinimumAge') which contains data about the minimum age of the population. The fourth field is Maximum Age (field name is 'MaximumAge') which contains data about the maximum age of the population. Next field is 'HealthyVolunteers' that have information about the requirements of healthy volunteers. The last field is 'Criteria' which has the inclusion and exclusion criteria for the trial.

Table 'tbl_studies_info' has columns: nct_id (VARCHAR), title (TEXT), description (TEXT), study_details (JSON), and foreign keys: . The table description is: Summary of various clinical studies including topics such as HIV, bone marrow collection, knee replacement, Cancer, and many healthcare problems.The table has four columns: nct_id (VARCHAR), title (TEXT), description (TEXT), and study_details (JSON). Every row displays a document.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'study_details' has information about the source of the document (field name is 'source') and the type of the study like Observational or Interventional(field name is 'study_type'), Enrollment details (field name is 'enrollment'), Status of the clinical trial(field name is 'status'), phase of the clinical trial(field name is 'phase'), and important dates(field name is 'date') like start date and update date. The information is in the json format

Table 'tbl_studies_adverse_details' has columns: id (VARCHAR), title (TEXT), description (TEXT), adverse_details (JSONB), and foreign keys: . The table description is: The table contains Summary information about reported adverse events (any untoward or unfavorable medical occurrence to participants, including abnormal physical exams, laboratory findings, symptoms, or diseases), including serious adverse events, other adverse events, and mortality. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.' title' is the title of the document.'description' is the description of the document. 'adverse_details' has information in JSON format. The first field is 'Event Type' (field name is 'EventType') which contains information about the type of adverse events like 'deaths' or 'serious'. The second field is about the count of subjects that are affected by the trial(field name is 'SubjestAffected'). The third field is 'Classification' which tells a detailed description of the Adverse Events.  The fourth field is about the count of subjects that are risked by the trial(field name is 'SubjectsRisk').
""",
    "OUTPUT": """SELECT si.title, si.study_details->>'enrollment' AS enrollment_count
FROM tbl_studies_info si
WHERE si.title = 'Recipient Vessels for Free Tissue Flaps in Advanced Oncologic Defects of the Midface and Scalp';
""",
},
{
    "QUESTION": " What is the purpose of the study Safety and Tolerability of RNS60 Given by IV to Healthy Subjects,anded",
    "CONTEXT": """Table 'tbl_studies_eligibilities' has columns: id (VARCHAR), title (TEXT), description (TEXT), eligibility_details (JSONB), and foreign keys: . The table description is: The table contains Information about the criteria used to select participants; includes inclusion and exclusion criteria. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'eligibility_details' has information in JSON format. The first field is 'Population' (field name is 'Population) which contains the information about the associated population type. The second field is the Sampling Method (field name is 'SamplingMethod') which contains the data about the method of sampling. The third field is Minimum Age (field name is 'MinimumAge') which contains data about the minimum age of the population. The fourth field is Maximum Age (field name is 'MaximumAge') which contains data about the maximum age of the population. Next field is 'HealthyVolunteers' that have information about the requirements of healthy volunteers. The last field is 'Criteria' which has the inclusion and exclusion criteria for the trial.

Table 'tbl_studies_adverse_details' has columns: id (VARCHAR), title (TEXT), description (TEXT), adverse_details (JSONB), and foreign keys: . The table description is: The table contains Summary information about reported adverse events (any untoward or unfavorable medical occurrence to participants, including abnormal physical exams, laboratory findings, symptoms, or diseases), including serious adverse events, other adverse events, and mortality. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.' title' is the title of the document.'description' is the description of the document. 'adverse_details' has information in JSON format. The first field is 'Event Type' (field name is 'EventType') which contains information about the type of adverse events like 'deaths' or 'serious'. The second field is about the count of subjects that are affected by the trial(field name is 'SubjestAffected'). The third field is 'Classification' which tells a detailed description of the Adverse Events.  The fourth field is about the count of subjects that are risked by the trial(field name is 'SubjectsRisk').

Table 'tbl_studies_info' has columns: nct_id (VARCHAR), title (TEXT), description (TEXT), study_details (JSON), and foreign keys: . The table description is: Summary of various clinical studies including topics such as HIV, bone marrow collection, knee replacement, Cancer, and many healthcare problems.The table has four columns: nct_id (VARCHAR), title (TEXT), description (TEXT), and study_details (JSON). Every row displays a document.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'study_details' has information about the source of the document (field name is 'source') and the type of the study like Observational or Interventional(field name is 'study_type'), Enrollment details (field name is 'enrollment'), Status of the clinical trial(field name is 'status'), phase of the clinical trial(field name is 'phase'), and important dates(field name is 'date') like start date and update date. The information is in the json format
""",
    "OUTPUT": """SELECT title, description, eligibility_details ->> 'Criteria' as criteria
FROM tbl_studies_eligibilities
WHERE title = 'Safety and Tolerability of RNS60 Given by IV to Healthy Subjects';
SQLResult: 
| title                                                      | description                                                                                                      | criteria                                                                                                      |
|------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|
| Safety and Tolerability of RNS60 Given by IV to Healthy Subjects | This study is designed to evaluate the safety and tolerability of RNS60 given by IV to healthy subjects.        | Inclusion Criteria: 1. Healthy male or female subjects between the ages of 18 and 55 years. Exclusion Criteria: 1. Any significant medical condition. |

Answer: The purpose of the study "Safety and Tolerability of RNS60 Given by IV to Healthy Subjects" is to evaluate the safety and tolerability of RNS60 given by IV to healthy subjects. The inclusion criteria include healthy male or female subjects between the ages of 18 and 55 years, while the exclusion criteria involve any significant medical condition.
""",
},
{
    "QUESTION": " What treatment was given to the participants in Intracoronary Injection of Epo After Myocardial Infarct ",
    "CONTEXT": """Table 'tbl_studies_info' has columns: nct_id (VARCHAR), title (TEXT), description (TEXT), study_details (JSON), and foreign keys: . The table description is: Summary of various clinical studies including topics such as HIV, bone marrow collection, knee replacement, Cancer, and many healthcare problems.The table has four columns: nct_id (VARCHAR), title (TEXT), description (TEXT), and study_details (JSON). Every row displays a document.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'study_details' has information about the source of the document (field name is 'source') and the type of the study like Observational or Interventional(field name is 'study_type'), Enrollment details (field name is 'enrollment'), Status of the clinical trial(field name is 'status'), phase of the clinical trial(field name is 'phase'), and important dates(field name is 'date') like start date and update date. The information is in the json format

Table 'tbl_studies_adverse_details' has columns: id (VARCHAR), title (TEXT), description (TEXT), adverse_details (JSONB), and foreign keys: . The table description is: The table contains Summary information about reported adverse events (any untoward or unfavorable medical occurrence to participants, including abnormal physical exams, laboratory findings, symptoms, or diseases), including serious adverse events, other adverse events, and mortality. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.' title' is the title of the document.'description' is the description of the document. 'adverse_details' has information in JSON format. The first field is 'Event Type' (field name is 'EventType') which contains information about the type of adverse events like 'deaths' or 'serious'. The second field is about the count of subjects that are affected by the trial(field name is 'SubjestAffected'). The third field is 'Classification' which tells a detailed description of the Adverse Events.  The fourth field is about the count of subjects that are risked by the trial(field name is 'SubjectsRisk').

Table 'tbl_studies_eligibilities' has columns: id (VARCHAR), title (TEXT), description (TEXT), eligibility_details (JSONB), and foreign keys: . The table description is: The table contains Information about the criteria used to select participants; includes inclusion and exclusion criteria. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'eligibility_details' has information in JSON format. The first field is 'Population' (field name is 'Population) which contains the information about the associated population type. The second field is the Sampling Method (field name is 'SamplingMethod') which contains the data about the method of sampling. The third field is Minimum Age (field name is 'MinimumAge') which contains data about the minimum age of the population. The fourth field is Maximum Age (field name is 'MaximumAge') which contains data about the maximum age of the population. Next field is 'HealthyVolunteers' that have information about the requirements of healthy volunteers. The last field is 'Criteria' which has the inclusion and exclusion criteria for the trial.
""",
    "OUTPUT": """SELECT title, description, study_details -> 'treatment' AS treatment
FROM tbl_studies_info
WHERE title = 'Intracoronary Injection of Epo After Myocardial Infarct "Intra-CO-EpoMI"'
SQLResult: title: Intracoronary Injection of Epo After Myocardial Infarct "Intra-CO-EpoMI", description: Study investigating the effects of intracoronary injection of Epo after myocardial infarction, treatment: Epo
Answer: The treatment given to the participants in the study "Intracoronary Injection of Epo After Myocardial Infarct "Intra-CO-EpoMI" was Epo.
""",
},
{
    "QUESTION": " What treatment was given to the participants in Exposure, Dose, Body Burden and Health Effects of Lead,anded",
    "CONTEXT": """Table 'tbl_studies_eligibilities' has columns: id (VARCHAR), title (TEXT), description (TEXT), eligibility_details (JSONB), and foreign keys: . The table description is: The table contains Information about the criteria used to select participants; includes inclusion and exclusion criteria. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'eligibility_details' has information in JSON format. The first field is 'Population' (field name is 'Population) which contains the information about the associated population type. The second field is the Sampling Method (field name is 'SamplingMethod') which contains the data about the method of sampling. The third field is Minimum Age (field name is 'MinimumAge') which contains data about the minimum age of the population. The fourth field is Maximum Age (field name is 'MaximumAge') which contains data about the maximum age of the population. Next field is 'HealthyVolunteers' that have information about the requirements of healthy volunteers. The last field is 'Criteria' which has the inclusion and exclusion criteria for the trial.

Table 'tbl_studies_adverse_details' has columns: id (VARCHAR), title (TEXT), description (TEXT), adverse_details (JSONB), and foreign keys: . The table description is: The table contains Summary information about reported adverse events (any untoward or unfavorable medical occurrence to participants, including abnormal physical exams, laboratory findings, symptoms, or diseases), including serious adverse events, other adverse events, and mortality. The table has the below columns.'nct_id' is the primary key of the table. It is the id of the document.' title' is the title of the document.'description' is the description of the document. 'adverse_details' has information in JSON format. The first field is 'Event Type' (field name is 'EventType') which contains information about the type of adverse events like 'deaths' or 'serious'. The second field is about the count of subjects that are affected by the trial(field name is 'SubjestAffected'). The third field is 'Classification' which tells a detailed description of the Adverse Events.  The fourth field is about the count of subjects that are risked by the trial(field name is 'SubjectsRisk').

Table 'tbl_studies_info' has columns: nct_id (VARCHAR), title (TEXT), description (TEXT), study_details (JSON), and foreign keys: . The table description is: Summary of various clinical studies including topics such as HIV, bone marrow collection, knee replacement, Cancer, and many healthcare problems.The table has four columns: nct_id (VARCHAR), title (TEXT), description (TEXT), and study_details (JSON). Every row displays a document.'nct_id' is the primary key of the table. It is the id of the document.'title' is the title of the document.'description' is the description of the document. 'study_details' has information about the source of the document (field name is 'source') and the type of the study like Observational or Interventional(field name is 'study_type'), Enrollment details (field name is 'enrollment'), Status of the clinical trial(field name is 'status'), phase of the clinical trial(field name is 'phase'), and important dates(field name is 'date') like start date and update date. The information is in the json format
""",
    "OUTPUT": """SELECT title, eligibility_details ->> 'Criteria' AS treatment
FROM tbl_studies_eligibilities
WHERE title = 'Exposure, Dose, Body Burden and Health Effects of Lead';
""",
},