RSpec.describe JPath do
  describe ".extract_from_string" do
    it "extracts ruby string from JSON" do
      pattern = "message.body"
      json_str = '{"message":{"body": "Hello World"}}'
      expect(JPath.extract_from_string(pattern, json_str)).to eq("Hello World")
    end

    it "extracts ruby object from JSON" do
      pattern = "message"
      json_str = '{"message":{"body": "Hello World"}}'
      expect(JPath.extract_from_string(pattern, json_str)).to eq({"body" => "Hello World"})
    end

    it "extracts complex hash from JSON" do
      pattern = "message"
      json_str = '{"message":{"body": "Hello World", "favorite_numbers": [42,0, 3.14159]}}'
      expect(JPath.extract_from_string(pattern, json_str)).to eq({"body" => "Hello World", "favorite_numbers" => [42, 0, 3.14159]})
    end
  end
end