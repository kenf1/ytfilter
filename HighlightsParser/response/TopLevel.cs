using System.Xml.Linq;

class TopLevelResponse
{
    public static XDocument ParseXml(string xmlContent)
    {
        return XDocument.Parse(xmlContent);
    }

    public static (XNamespace atom, XNamespace media) GetNamespaces()
    {
        return (
            atom: "http://www.w3.org/2005/Atom",
            media: "http://search.yahoo.com/mrss/"
        );
    }
}