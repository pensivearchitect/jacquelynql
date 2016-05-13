class AddMatchesToPlayers < ActiveRecord::Migration[5.0]
  def change
    change_table :players do |t|
      t.references :matches, foreign_key: true
    end
  end
end
