from database.db import db


class Task(db.DynamicDocument):
    task_id = db.StringField(required=True)


class Result(db.DynamicDocument):
    result_id = db.StringField(required=True)


class TaskHistory(db.DynamicDocument):
    task_object = db.StringField()
