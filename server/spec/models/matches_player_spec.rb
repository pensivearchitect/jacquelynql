require 'rails_helper'

RSpec.describe MatchesPlayer, type: :model do
  it { should belong_to(:match) }
  it { should belong_to(:player) }
end
