import DS from 'ember-data';

export default DS.Model.extend({
  matches: DS.hasMany('match'),
  name: DS.attr('string'),
  // previousNames: DS.attr('array'),
  steamId: DS.attr('string')
});
