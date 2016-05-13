require 'rails_helper'

RSpec.describe Player, type: :model do
  it { should have_many(:matches).through(:matches_player) }
end
