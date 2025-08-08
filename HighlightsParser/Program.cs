using System.Text.Json;
using System.Xml.Linq;

class Program
{
    static async Task Main()
    {
        string url = "https://www.youtube.com/feeds/videos.xml?channel_id=UC9ckyA_A3MfXUa0ttxMoIZw";

        try
        {
            string xmlContent = await Request.FetchXmlAsync(url);
            XDocument doc = Request.ParseXml(xmlContent);

            var ns = XmlNamespaces.GetNamespaces();
            IEnumerable<XElement> entries = Contents.GetAllEntries(doc, ns.atom);

            if (!entries.Any())
            {
                Console.WriteLine("No entries found in XML.");
            }
            else
            {
                var entryDataList = new List<EntryInfo>();

                foreach (var entry in entries)
                {
                    if (Contents.CheckEntryEmpty(entry))
                    {
                        EntryInfo entryData = Contents.GetEntryData(entry, ns);
                        entryDataList.Add(entryData);
                    }
                }

                var jsonOptions = new JsonSerializerOptions
                {
                    WriteIndented = true,
                    PropertyNamingPolicy = JsonNamingPolicy.CamelCase
                };

                string jsonString = JsonSerializer.Serialize(entryDataList, jsonOptions);

                File.WriteAllText("../data/entries.json", jsonString);

                Console.WriteLine($"Saved {entryDataList.Count} entries to ../data/entries.json");
            }
        }
        catch (HttpRequestException e)
        {
            Console.WriteLine("Error fetching XML: " + e.Message);
            Environment.Exit(1);
        }
        catch (System.Xml.XmlException e)
        {
            Console.WriteLine("Error parsing XML: " + e.Message);
            Environment.Exit(1);
        }
    }
}
