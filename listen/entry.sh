#!/usr/bin/env bash
mongod&

cd listen

python3 app.py