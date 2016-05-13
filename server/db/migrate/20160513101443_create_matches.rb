class CreateMatches < ActiveRecord::Migration[5.0]
  def change
    create_table :matches do |t|
      t.references :players, foreign_key: true
      t.string :match_guid
      t.string :server_name
      t.integer :game_length
      t.string :factory

      t.timestamps
    end
  end
end
