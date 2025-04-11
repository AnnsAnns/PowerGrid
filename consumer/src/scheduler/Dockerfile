FROM python:3.9.2-alpine3.13

WORKDIR /app

COPY . .

RUN pip install --no-cache-dir -r requirements.txt

CMD [ "python", "run.py" ]
