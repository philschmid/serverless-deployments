import os
import gradio as gr

from transformers import pipeline

print("loading model")
print(os.listdir("model"))
# clf = pipeline("sentiment-analysis", model="model/")


def sentiment(payload):
    # prediction = clf(payload, return_all_scores=True)

    # # convert list to dict
    # result = {}
    # for pred in prediction[0]:
    #     result[pred["label"]] = pred["score"]
    # return result
    return {"test": 1.0}


demo = gr.Interface(
    fn=sentiment,
    inputs=gr.Textbox(placeholder="Enter a positive or negative sentence here..."),
    outputs="label",
    interpretation="default",
    examples=[["This is wonderful!"]],
    allow_flagging="never",
)

demo.launch(
    server_port=8080,
    # server_name="0.0.0.0",
    enable_queue=False,
)
