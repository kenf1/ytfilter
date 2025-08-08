using System.Xml.Linq;

class EntryParseWrapper
{
    public static async Task Run(string url)
    {
        try
        {
            string xmlContent = await GetRequest.FetchXmlAsync(url);
            var entryDataList = ProcessXmlEntries(xmlContent);

            JsonHelper.SaveListToJsonFile(entryDataList, "../data/entries.json");
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

    private static IEnumerable<EntryInfo> ProcessXmlEntries(string xmlContent)
    {
        XDocument doc = TopLevelResponse.ParseXml(xmlContent);

        //namespaces = key
        var ns = TopLevelResponse.GetNamespaces();
        IEnumerable<XElement> entries = EntryField.GetAllEntries(doc, ns.atom);

        if (!entries.Any())
        {
            Console.WriteLine("No entries found in XML.");
            return Enumerable.Empty<EntryInfo>();
        }

        return EntryContents.ExtractEntryDataList(entries, ns);
    }
}