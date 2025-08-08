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
                Contents.DisplayEntryDetails(entry, ns);
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
