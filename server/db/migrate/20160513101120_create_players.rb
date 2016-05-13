class CreatePlayers < ActiveRecord::Migration[5.0]
  def change
    create_table :players do |t|
      t.string :name
      t.string :previous_names
      t.string :steam_id

      t.timestamps
    end
  end
end
