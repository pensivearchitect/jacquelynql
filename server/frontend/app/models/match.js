import DS from 'ember-data';

export default DS.Model.extend({
  matchGuid: DS.attr('string'),
  serverName: DS.attr('string'),
  players: DS.hasMany('player'),
  gameLength: DS.attr('number'),
  gameType: DS.attr('string')
});
