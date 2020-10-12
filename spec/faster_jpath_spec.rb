RSpec.describe FasterJpath do
  it "has a version number" do
    expect(FasterJpath::VERSION).not_to be nil
  end

  it "initializes properly" do
    expect(JPath.say_rust_hello).to eq({"message" => "hello world"})
  end
  
end
