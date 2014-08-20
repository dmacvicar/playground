
# MiniManager

MiniManager is a systems management application built on top
of SaltStack.

Unlike Halite, It does not pretend to be a Salt frontend, but
to offer familiar Spacewalk user interface using the Salt stack
as the agent.

## Design

The current design decisions have been made:

* The application runs on the salt master.
  It could as well use the salt-api
  * Using salt-api means either to have the browser use the salt-api
    directly, cross-origin problem
  * ... or to wrap every salt-api call on the backend: overhead
  * therefore we use the python API on the client and expose simple
    browser-oriented json APIs for the javascript parts

* There is no database
  * no need until now... may be in the future use returners to cache
    data from minions (redis??)
  * may be like Spacewalk, a task scheduler collecting data from
    minions and caching it.

* We could use websockets for everything however not all Salt data is
  real time, so we use old-AJAX for that.

The current stuff I am not sure about:

* Collect data and have the web app operate in a cache.
  One could use the scheduler and returners. See 9.8 in
  http://docs.saltstack.com/en/latest/topics/jobs
  The problem is that returners run on the minions.

  Also see https://speakerdeck.com/ipmb/monitoring-infrastructure-with-saltstack

  

## Issues

* you need to run the app as root (https://github.com/saltstack/salt/issues/14918)

## Development

It is implemented using Flask web framework running on top of Tornado
to implement websockets.

On the client side, it uses jQuery and Facebook's React.

## Templates

