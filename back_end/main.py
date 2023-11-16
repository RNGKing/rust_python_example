import qr_code_parser as parser
import base64
from flask import Flask, jsonify, send_from_directory, request
import os
from flask_cors import CORS, cross_origin
from codecs import encode
import json

app = Flask(__name__, static_folder='app', static_url_path="/app")
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

@app.route("/heartbeat")
def heartbeat():
    return jsonify({"status": "healthy"})

@app.route("/send_qr", methods=["POST"])
@cross_origin()
def send_qr():
    data = request.get_json()["image"]
    out_bytes = base64.decodebytes(bytes(data,'utf-8'))
    with open("./testing_output.png", "wb") as fh:
        fh.write(out_bytes)
    result = parser.process_image_from_bytes(out_bytes)
    print(result)
    return jsonify({"status": "ok"})


@app.route('/', defaults={'path': ''})
@app.route('/<path:path>')
def serve(path):
    if path != "" and os.path.exists(app.static_folder + '/' + path):
        return send_from_directory(app.static_folder, path)
    else:
        return send_from_directory(app.static_folder, 'index.html')

# create the file to be used
#parser.create_qr_code_from_string("this is a test", "./testing_out.png")

#f = open('./testing_out.png', mode='rb')
#data = f.read()
#print(type(data))
#result = parser.process_image_from_bytes(data)
#print(result)
#f.close()