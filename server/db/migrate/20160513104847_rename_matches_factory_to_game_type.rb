class RenameMatchesFactoryToGameType < ActiveRecord::Migration[5.0]
  def change
    rename_column :matches, :factory, :game_type
  end
end
