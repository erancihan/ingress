from flask import Flask, jsonify, request

app = Flask(__name__)


@app.route("/", methods=["GET", "POST", "PUT", "DELETE"])
@app.route("/<path:rest>", methods=["GET", "POST", "PUT", "DELETE"])
def hello_world(*args, **kwargs):
    response = {
        "message": "Response from proxy",
        "method": request.method,
        "url": f"{request.url}",
    }

    return jsonify(response)


if __name__ == "__main__":
    app.run(port=8081, debug=True)
