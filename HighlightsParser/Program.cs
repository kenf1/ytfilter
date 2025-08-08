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
            XElement entry = Contents.GetFirstEntry(doc, ns.atom)!;

            if (Contents.CheckEntryEmpty(entry) == true)
            {
                EntryInfo entryData = Contents.GetEntryData(entry, ns);

                var jsonOptions = new JsonSerializerOptions
                {
                    WriteIndented = true,
                    PropertyNamingPolicy = JsonNamingPolicy.CamelCase
                };

                string jsonString = JsonSerializer.Serialize(entryData, jsonOptions);

                Console.WriteLine(jsonString);

                //save to file
                System.IO.File.WriteAllText("../data/entry.json", jsonString);
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
