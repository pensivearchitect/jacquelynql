class DropReferencesFromPlayersAndMatches < ActiveRecord::Migration[5.0]
  def change
    remove_column :players, :matches_id
    remove_column :matches, :players_id
  end
end
