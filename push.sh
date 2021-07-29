#! /bin/bash

mkdir ../app
cp target/release/udbot ../app
cp scripts/getapidata.sh ../app
cp heroku.yml ../app
cp Dockerfile ../app
cd ../app
git config --global user.name "idkwhoiam322"
git config --global user.email "idkwhoiam322@raphielgang.org"
git init
git add .
git commit -m "Push Built Binary"
git push -f https://git.heroku.com/idkwhoiam-udbot.git HEAD:master
cd ../udbot