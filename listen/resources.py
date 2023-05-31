import uuid
import json

from flask import request, Response
from flask_restful import Resource
from database.models import Task, Result, TaskHistory


class Tasks(Resource):
    # ListTasks
    def get(self):
        tasks = Task.objects().to_json()
        return Response(tasks, mimetype="application/json", status=200)

    # AddTasks
    def post(self):
        body = request.get_json()
        json_obj = json.loads(json.dumps(body))
        obj_num = len(body)
        for i in range(obj_num):
            json_obj[i]["task_id"] = str(uuid.uuid4())
            Task(**json_obj[i]).save()
            task_options = []
            for key in json_obj[i].keys():
                if key != "task_type" and key != "task_id":
                    task_options.append(key + ": " + json_obj[i][key])
            TaskHistory(
                task_id=json_obj[i]["task_id"],
                task_type=json_obj[i]["task_type"],
                task_object=json.dumps(json_obj),
                task_options=task_options,
                task_results="",
            ).save()
        return Response(
            Task.objects.skip(Task.objects.count() - obj_num).to_json(),
            mimetype="application/json",
            status=200,
        )


class Results(Resource):
    # ListResults
    def get(self):
        results = Result.objects().to_json()
        return Response(results, mimetype="application.json", status=200)

    # AddResults
    def post(self):
        if str(request.get_json()) != "[]":
            body = request.get_json()
            print("Received implant response: {}".format(body))
            json_obj = json.loads(json.dumps(body))

            for ob in json_obj:
                ob["result_id"] = str(uuid.uuid4())
                Result(**ob).save()

            tasks = Task.objects().to_json()
            Task.objects().delete()
            return Response(tasks, mimetype="application/json", status=200)
        else:
            tasks = Task.objects().to_json()
            Task.objects().delete()
            return Response(tasks, mimetype="application/json", status=200)


class History(Resource):
    # ListHistory
    def get(self):
        task_history = TaskHistory.objects().to_json()
        results = Result.objects().to_json()
        json_obj = json.loads(results)
        result_obj_collection = []
        for i in range(len(json_obj)):
            for field in json_obj[i]:
                result_obj = {"task_id": field, "task_results": json_obj[i][field]}
                result_obj_collection.append(result_obj)
        for result in result_obj_collection:
            if TaskHistory.objects(task_id=result["task_id"]):
                TaskHistory.objects(task_id=result["task_id"]).update_one(
                    set__task_results=result["task_results"]
                )
        return Response(task_history, mimetype="application/json", status=200)
