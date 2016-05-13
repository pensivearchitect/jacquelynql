class CreateJoinTableMatchesPlayers < ActiveRecord::Migration[5.0]
  def change
    create_join_table :matches, :players
  end
end
