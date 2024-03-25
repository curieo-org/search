#%%
import dspy
# setting up and testing the basic signature
class Router_moduleQA(dspy.Signature):
    "Study the `question` thoroughly to understand the context and meaning of the query before generating the route to specific route.Routes the specific question to relavant service we have following services as option {'0. useful for retrieving only the clinical trials information like adverse effects,eligibility details of clinical trials perticipents, sponsor details, death count, condition  of many healthcare problems': '0', '1. useful for retrieving general information about healthcare data. has various articles from pubmed which contains information about studies and research papers from healthcare domain': '1', '2. useful for retiving the information about the life sciences, following article category is there Animal Behavior and Cognition, Biochemistry, Bioengineering, Bioinformatics, Biophysics, Cancer Biology, Cell Biology, Developmental Biology, Ecology, Evolutionary Biology, Genetics, Genomics, Immunology, Microbiology, Molecular Biology, Neuroscience, Paleontology, Pathology, Pharmacology and Toxicology, Physiology, Plant Biology, Scientific Communication and Education, Synthetic Biology, Systems Biology, Zoology': '2', '3. useful only for retrieving the drug related information like molecular weights,similarities,smile codes, target medicines, effects on other medicine': '3'}"
    question = dspy.InputField(desc = "Question to be routed to route")
    answer = dspy.OutputField(desc="route option should be integer without any additional explaination", prefix = "The route number to the question is:")

class Router_module(dspy.Module):
    "Routes the specific question to relavant service we have following services as option {'0. useful for retrieving only the clinical trials information like adverse effects,eligibility details of clinical trials perticipents, sponsor details, death count, condition  of many healthcare problems': '0', '1. useful for retrieving general information about healthcare data. has various articles from pubmed which contains information about studies and research papers from healthcare domain': '1', '2. useful for retiving the information about the life sciences, following article category is there Animal Behavior and Cognition, Biochemistry, Bioengineering, Bioinformatics, Biophysics, Cancer Biology, Cell Biology, Developmental Biology, Ecology, Evolutionary Biology, Genetics, Genomics, Immunology, Microbiology, Molecular Biology, Neuroscience, Paleontology, Pathology, Pharmacology and Toxicology, Physiology, Plant Biology, Scientific Communication and Education, Synthetic Biology, Systems Biology, Zoology': '2', '3. useful only for retrieving the drug related information like molecular weights,similarities,smile codes, target medicines, effects on other medicine': '3'}"
    def __init__(self):
        super().__init__()
        self.generate_answer = dspy.ChainOfThought(Router_moduleQA)
    
    def forward(self, question):
        prediction = self.generate_answer(question=question)
        return dspy.Prediction(answer=prediction.answer)
    
mod = Router_module()
# mod.load('/Users/som/Downloads/code/search/backend/app/dspy_integration/dspy_programs/orchestrator_router_prompt.json')
# lm = dspy.OpenAI(api_key="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")
# dspy.settings.configure(lm =lm)
# mod.forward("hi")
# %%
