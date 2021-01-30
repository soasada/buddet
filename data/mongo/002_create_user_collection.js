db = db.getSiblingDB('buddetdb'); // like 'use buddetdb'
db.createCollection('user');
db.user.createIndex({'email': 1}, {unique: true});