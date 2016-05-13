class PlayerSerializer < ActiveModel::Serializer
  attributes :id, :name, :previous_names, :steam_id
  has_many :matches
end
