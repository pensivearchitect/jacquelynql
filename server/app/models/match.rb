class Match < ApplicationRecord
  has_many :matches_player
  has_many :players, through: :matches_player
end
