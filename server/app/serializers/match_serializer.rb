class MatchSerializer < ActiveModel::Serializer
  attributes :id, :match_guid, :server_name, :game_length, :game_type
  has_many :players
end
