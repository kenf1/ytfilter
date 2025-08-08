using System.Xml.Linq;

class XmlNamespaces
{
    public static (XNamespace atom, XNamespace media) GetNamespaces()
    {
        return (
            atom: "http://www.w3.org/2005/Atom",
            media: "http://search.yahoo.com/mrss/"
        );
    }
}