# This script generates jsons that contain
# logger url using Heroku API.

# Remove deprecated jsons
rm -rf *.json;

curl -o "worker_log_details.json" -n -X POST https://api.heroku.com/apps/idkwhoiam-udbot/log-sessions \
  -d '{
  "dyno": "worker",
  "lines": 1500,
  "source": "app"
}' \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3" \
  -H "Authorization: Bearer $HEROKU_API_KEY";

curl -o "api_log_details.json" -n -X POST https://api.heroku.com/apps/idkwhoiam-udbot/log-sessions \
  -d '{
  "dyno": "api",
  "lines": 1500,
  "source": "app"
}' \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3" \
  -H "Authorization: Bearer $HEROKU_API_KEY";

curl -o "heroku_worker_log_details.json" -n -X POST https://api.heroku.com/apps/idkwhoiam-udbot/log-sessions \
  -d '{
  "dyno": "worker",
  "lines": 1500,
  "source": "heroku"
}' \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3" \
  -H "Authorization: Bearer $HEROKU_API_KEY";
