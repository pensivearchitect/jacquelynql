import { moduleForModel, test } from 'ember-qunit';

moduleForModel('match', 'Unit | Model | match', {
  // Specify the other units that are required for this test.
  needs: ['model:player']
});

test('it exists', function(assert) {
  let model = this.subject();
  // let store = this.store();
  assert.ok(!!model);
});
