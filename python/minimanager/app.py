from flask import Flask, request, session, g, redirect, url_for, abort, \
     render_template, flash
import requests
import json

# create our little application :)
app = Flask(__name__, static_url_path='')
app.config.from_object(__name__)

app.config.from_envvar('FLASKR_SETTINGS', silent=True)

SALT_API = 'http://localhost:8001'

@app.route('/')
def index():
    if 'salt-token' in session:
        print('Token is %s' % session['salt-token'])
    else:
        return redirect(url_for('login'))

    headers = {'Accept': 'application/json', 'Content-Type': 'application/json'}
    headers['X-Auth-Token'] = session['salt-token']

    r = requests.get(SALT_API + '/minions', headers=headers)
    print r.status_code
    print r.text

    return render_template('index.html')

@app.route('/minions')
def minions():
    if 'salt-token' in session:
        print('Token is %s' % session['salt-token'])
    else:
        return redirect(url_for('login'))
    headers = {'Accept': 'application/json', 'Content-Type': 'application/json'}
    headers['X-Auth-Token'] = session['salt-token']

    r = requests.get(SALT_API + '/minions', headers=headers)
    print r.status_code

    minionmap = r.json()['return'][0]
    print(json.dumps(minionmap))
    # transform { minionid: {prop1: 'val1', ..}, ...} into
    # [{id: 'minionid', prop1: 'val1', ...}, ...]
    minions = [dict(v, id=k) for k,v in minionmap.items()]
    return json.dumps(minions)


@app.route('/login', methods = ['GET', 'POST'])
def login():
    if request.method == 'POST':
        username = request.form['username']
        password = request.form['password']
        data = {'eauth': 'pam', 'username': username,'password': password}
        headers = {'Accept': 'application/json', 'Content-Type': 'application/json'}
        print(json.dumps(data))
        r = requests.post(SALT_API + '/login', data=json.dumps(data), headers=headers)
        if r.status_code == 200:
            flash("Logged in successfully.")
            print(r.text)
            session['salt-token'] = r.json()['return'][0]['token']
            return redirect(url_for('index'))

        flash("Login incorrect: %s" % r.text)
        return redirect(url_for('login'))
    return render_template('login.html')

@app.route('/logout')
def logout():
    session.pop('salt-token', None)
    return redirect(url_for('index'))

if __name__ == '__main__':
    app.secret_key = '\x9e\x16\x12(yY\x1c\xacN\xbe\xad\x08\x08\x8a\x19\x9cj\xf0\xe8\x9d\x99l\xd2k'

    app.run(debug=True)
