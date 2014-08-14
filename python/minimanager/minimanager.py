from functools import wraps
from flask import Flask, request, session, g, redirect, url_for, abort, \
     render_template, flash, jsonify
import requests
import json
from salt.client.api import APIClient

# create our little application :)
app = Flask(__name__, static_url_path='')
app.secret_key = '\x9e\x16\x12(yY\x1c\xacN\xbe\xad\x08\x08\x8a\x19\x9cj\xf0\xe8\x9d\x99l\xd2k'

app.config.from_object(__name__)

app.config.from_envvar('FLASKR_SETTINGS', silent=True)

SALT_API = 'http://localhost:8001'

def login_required(f):
    @wraps(f)
    def decorated_function(*args, **kwargs):
        app.logger.info(request.url)
        if not 'salt-token' in session:
            return redirect(url_for('login', next=request.url))
        app.logger.info('Token is %s' % session['salt-token'])
        return f(*args, **kwargs)
    return decorated_function

def _auth_token():
    return session['salt-token']

@app.route('/')
def index():
    return redirect(url_for('minions'))

@app.route('/login', methods = ['GET', 'POST'])
def login():
    if request.method == 'POST':
        username = request.form['username']
        password = request.form['password']
        data = {'eauth': 'pam', 'username': username,'password': password}
        client = APIClient()
        try:
            ret = client.create_token(data)
            print(ret)
            session['salt-token'] = ret['token']
            flash("Logged in successfully.")
            return redirect(url_for('index'))
        except Exception as e:
            flash("Login incorrect: %s" % e)
            return redirect(url_for('login'))
    return render_template('login.html')

@app.route('/logout')
def logout():
    session.pop('salt-token', None)
    return redirect(url_for('index'))

@app.route('/api/minions')
@app.route('/api/minions/<id>')
@login_required
def ajax_minions(id=None):
    client = APIClient()
    app.logger.info("auth: %s" % _auth_token())
    ret = client.minion_sync(tgt=(id or '*'), fun='grains.items', token=_auth_token(), timeout=1000)
    # transform { minionid: {prop1: 'val1', ..}, ...} into
    # [{id: 'minionid', prop1: 'val1', ...}, ...]
    minions = [dict(v, id=k) for k,v in ret.items()]

    if id is None:
        return json.dumps(minions)

    if len(minions) > 0:
        minion = minions[0]
        print(minion)
        return json.dumps(minion)

    return abort(404)

@app.route('/minions')
@login_required
def minions():
    return render_template('minions.html')

@app.route('/minions/<id>')
@login_required
def minion(id):
    return render_template('minion.html', id=id)

if __name__ == '__main__':
    app.run(debug=True)
