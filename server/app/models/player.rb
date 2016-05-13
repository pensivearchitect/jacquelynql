class Player < ApplicationRecord
  has_many :matches_player
  has_many :matches, through: :matches_player
end
