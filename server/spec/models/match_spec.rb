require 'rails_helper'

RSpec.describe Match, type: :model do
  it { should have_many(:players).through(:matches_player) }
end
