from flask import render_template

MSG_TYPE_SEARCH_STEP = 'search-step'
MSG_TYPE_OPEN_AI_STREAM = 'openai-stream'

# global var to store progress. Native polling 'socket'
exporting_progress = {}


class Sender:
    def __init__(self, request_id: str):
        self.request_id = request_id
        self.received_step_events = []
        self.search_result_step_html = ''

    def send_message(self, msg_type, msg: str):
        self.received_step_events.append(msg)
