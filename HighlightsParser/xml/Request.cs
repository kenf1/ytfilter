using System.Xml.Linq;

class Request
{
    public static async Task<string> FetchXmlAsync(string url)
    {
        using HttpClient client = new HttpClient();
        return await client.GetStringAsync(url);
    }

    public static XDocument ParseXml(string xmlContent)
    {
        return XDocument.Parse(xmlContent);
    }
}