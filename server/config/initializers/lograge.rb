Server::Application.configure do
  config.lograge.enabled = true
  config.lograge.formatter = Lograge::Formatters::Json.new
  config.lograge.custom_options = -> (event) do
    { time: event.time.to_s }
  end
end
