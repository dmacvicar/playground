
var Manager = angular.module('Manager', ['ngResource']);

Manager.factory('Minions', function ($resource) {
  var obj = {};
  obj.all = $resource('minions', {}, {
      query: {
        method: 'GET',
        params: {},
        isArray: true
      }
    });
  return obj;
});

Manager.controller('MinionsController', function MinionsController($scope, Minions) {
  Minions.all.query(function (response) {
    $scope.all = response;
    console.log('foo! ' + response);
  });
});
