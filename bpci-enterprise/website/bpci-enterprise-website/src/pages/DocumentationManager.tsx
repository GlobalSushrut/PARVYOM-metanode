import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import { Textarea } from '../components/ui/textarea';
import { Badge } from '../components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../components/ui/tabs';
import { 
  FileText, 
  Download, 
  Upload, 
  Trash2, 
  Edit, 
  Save, 
  X, 
  Plus,
  Eye,
  Code,
  BookOpen,
  Settings,
  RefreshCw
} from 'lucide-react';

interface DocumentFile {
  id: string;
  name: string;
  content: string;
  size: number;
  lastModified: string;
  description: string;
}

const DocumentationManager: React.FC = () => {
  const [documents, setDocuments] = useState<DocumentFile[]>([]);
  const [selectedDoc, setSelectedDoc] = useState<DocumentFile | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [editContent, setEditContent] = useState('');
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [showAddForm, setShowAddForm] = useState(false);
  const [newDocName, setNewDocName] = useState('');
  const [newDocContent, setNewDocContent] = useState('');
  const [newDocDescription, setNewDocDescription] = useState('');

  // Initial booklet documents data
  const initialDocuments: DocumentFile[] = [
    {
      id: '01',
      name: '01-MATHEMATICAL_FOUNDATIONS.md',
      content: '# 🧮 Mathematical Foundations\n\nRevolutionary 6D blockchain mathematics with quantum resistance...',
      size: 27133,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Revolutionary 6D blockchain mathematics with quantum-resistant cryptography'
    },
    {
      id: '02',
      name: '02-THEORETICAL_FRAMEWORK.md',
      content: '# 🧠 Theoretical Framework\n\nAdvanced 6D blockchain theory with consensus frameworks...',
      size: 22994,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Advanced 6D blockchain theory with multi-dimensional consensus frameworks'
    },
    {
      id: '03',
      name: '03-CORE_INFRASTRUCTURE_STACK.md',
      content: '# 🏗️ Core Infrastructure Stack\n\nComplete advanced code stack analysis...',
      size: 21226,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Complete infrastructure stack with Rust ecosystem and advanced technologies'
    },
    {
      id: '04',
      name: '04-ARCHITECTURE_OVERVIEW.md',
      content: '# 🏛️ Architecture Overview\n\nRevolutionary Pravyom/Metanode 6D architecture...',
      size: 47544,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Complete system architecture with foundational layers and production infrastructure'
    },
    {
      id: '05',
      name: '05-BREAKTHROUGH_DISCOVERY.md',
      content: '# 🚀 Breakthrough Discovery\n\nSeven major production breakthroughs...',
      size: 29582,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Seven major breakthroughs with systematic validation evidence'
    },
    {
      id: '06',
      name: '06-SAAS_MIGRATION.md',
      content: '# 🔄 SaaS Migration\n\nAdvanced SaaS application migration to native 6D Pravyom client...',
      size: 42265,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Advanced SaaS migration with Web3.5 architecture integration'
    },
    {
      id: '07',
      name: '07-BPI_OS_DOWNLOAD_USAGE_GUIDE.md',
      content: '# 🚀 BPI OS Download & Deep Usage Guide\n\nComprehensive guide for downloading BPI OS...',
      size: 14911,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Complete BPI OS download, installation, and deep usage guide'
    },
    {
      id: '08',
      name: '08-BPCI_ALL_SERVERS_DOCUMENTATION.md',
      content: '# 🏭 BPCI All Servers Comprehensive Documentation\n\nAll 14 BPCI servers documentation...',
      size: 17663,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Comprehensive documentation of all 14 BPCI servers with API endpoints'
    },
    {
      id: '09',
      name: '09-BPI_OS_COMPLETE_DOCUMENTATION.md',
      content: '# 🌟 PRAVYOM BPI OS: Complete Technical Documentation\n\nRevolutionary blockchain infrastructure...',
      size: 46912,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Large comprehensive BPI OS documentation with 9-layer architecture'
    },
    {
      id: '10',
      name: '10-BPCI_SERVERS_COMPLETE_OVERVIEW.md',
      content: '# 🖥️ BPCI Servers Complete Overview\n\nBPI server architecture and implementation...',
      size: 9572,
      lastModified: '2024-11-03T22:00:00Z',
      description: 'Complete BPI server documentation and architecture overview'
    }
  ];

  useEffect(() => {
    // Simulate loading from API
    setTimeout(() => {
      setDocuments(initialDocuments);
      setLoading(false);
    }, 1000);
  }, []);

  const filteredDocuments = documents.filter(doc =>
    doc.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    doc.description.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const formatDate = (dateString: string): string => {
    return new Date(dateString).toLocaleString();
  };

  const handleDocumentSelect = (doc: DocumentFile) => {
    setSelectedDoc(doc);
    setEditContent(doc.content);
    setIsEditing(false);
  };

  const handleSaveEdit = () => {
    if (selectedDoc) {
      const updatedDocuments = documents.map(doc =>
        doc.id === selectedDoc.id
          ? { ...doc, content: editContent, lastModified: new Date().toISOString() }
          : doc
      );
      setDocuments(updatedDocuments);
      setSelectedDoc({ ...selectedDoc, content: editContent });
      setIsEditing(false);
    }
  };

  const handleDeleteDocument = (docId: string) => {
    if (confirm('Are you sure you want to delete this document?')) {
      setDocuments(documents.filter(doc => doc.id !== docId));
      if (selectedDoc?.id === docId) {
        setSelectedDoc(null);
      }
    }
  };

  const handleAddDocument = () => {
    if (newDocName && newDocContent) {
      const newDoc: DocumentFile = {
        id: Date.now().toString(),
        name: newDocName.endsWith('.md') ? newDocName : `${newDocName}.md`,
        content: newDocContent,
        size: new Blob([newDocContent]).size,
        lastModified: new Date().toISOString(),
        description: newDocDescription || 'New documentation file'
      };
      setDocuments([...documents, newDoc]);
      setNewDocName('');
      setNewDocContent('');
      setNewDocDescription('');
      setShowAddForm(false);
    }
  };

  const handleDownloadDocument = (doc: DocumentFile) => {
    const blob = new Blob([doc.content], { type: 'text/markdown' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = doc.name;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const handleDownloadAll = () => {
    documents.forEach(doc => {
      setTimeout(() => handleDownloadDocument(doc), 100);
    });
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 flex items-center justify-center">
        <div className="text-center">
          <RefreshCw className="h-8 w-8 animate-spin text-purple-400 mx-auto mb-4" />
          <p className="text-white">Loading Documentation...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 p-6">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h1 className="text-4xl font-bold text-white mb-2">
                📚 Pravyom Documentation Manager
              </h1>
              <p className="text-purple-200">
                Manage all booklet markdown files • View, Edit, Add, Remove documentation
              </p>
            </div>
            <div className="flex gap-3">
              <Button
                onClick={handleDownloadAll}
                className="bg-green-600 hover:bg-green-700"
              >
                <Download className="h-4 w-4 mr-2" />
                Download All
              </Button>
              <Button
                onClick={() => setShowAddForm(true)}
                className="bg-purple-600 hover:bg-purple-700"
              >
                <Plus className="h-4 w-4 mr-2" />
                Add Document
              </Button>
            </div>
          </div>

          {/* Stats */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
            <Card className="bg-slate-800/50 border-purple-500/20">
              <CardContent className="p-4">
                <div className="flex items-center">
                  <BookOpen className="h-8 w-8 text-purple-400 mr-3" />
                  <div>
                    <p className="text-2xl font-bold text-white">{documents.length}</p>
                    <p className="text-purple-200 text-sm">Total Documents</p>
                  </div>
                </div>
              </CardContent>
            </Card>
            <Card className="bg-slate-800/50 border-purple-500/20">
              <CardContent className="p-4">
                <div className="flex items-center">
                  <FileText className="h-8 w-8 text-blue-400 mr-3" />
                  <div>
                    <p className="text-2xl font-bold text-white">
                      {formatFileSize(documents.reduce((acc, doc) => acc + doc.size, 0))}
                    </p>
                    <p className="text-purple-200 text-sm">Total Size</p>
                  </div>
                </div>
              </CardContent>
            </Card>
            <Card className="bg-slate-800/50 border-purple-500/20">
              <CardContent className="p-4">
                <div className="flex items-center">
                  <Settings className="h-8 w-8 text-green-400 mr-3" />
                  <div>
                    <p className="text-2xl font-bold text-white">100%</p>
                    <p className="text-purple-200 text-sm">Complete</p>
                  </div>
                </div>
              </CardContent>
            </Card>
            <Card className="bg-slate-800/50 border-purple-500/20">
              <CardContent className="p-4">
                <div className="flex items-center">
                  <Code className="h-8 w-8 text-yellow-400 mr-3" />
                  <div>
                    <p className="text-2xl font-bold text-white">6D</p>
                    <p className="text-purple-200 text-sm">Blockchain Tech</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Search */}
          <div className="mb-6">
            <Input
              placeholder="Search documentation files..."
              value={searchTerm}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setSearchTerm(e.target.value)}
              className="bg-slate-800/50 border-purple-500/20 text-white placeholder-purple-300"
            />
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Document List */}
          <div className="lg:col-span-1">
            <Card className="bg-slate-800/50 border-purple-500/20">
              <CardHeader>
                <CardTitle className="text-white flex items-center">
                  <FileText className="h-5 w-5 mr-2" />
                  Documentation Files
                </CardTitle>
              </CardHeader>
              <CardContent className="p-0">
                <div className="max-h-96 overflow-y-auto">
                  {filteredDocuments.map((doc) => (
                    <div
                      key={doc.id}
                      className={`p-4 border-b border-slate-700 cursor-pointer hover:bg-slate-700/50 transition-colors ${
                        selectedDoc?.id === doc.id ? 'bg-purple-900/30' : ''
                      }`}
                      onClick={() => handleDocumentSelect(doc)}
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <h4 className="text-white font-medium text-sm mb-1">{doc.name}</h4>
                          <p className="text-purple-200 text-xs mb-2">{doc.description}</p>
                          <div className="flex items-center gap-2">
                            <Badge variant="secondary" className="text-xs">
                              {formatFileSize(doc.size)}
                            </Badge>
                            <span className="text-purple-300 text-xs">
                              {formatDate(doc.lastModified)}
                            </span>
                          </div>
                        </div>
                        <div className="flex gap-1 ml-2">
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleDownloadDocument(doc);
                            }}
                            className="h-6 w-6 p-0 text-purple-400 hover:text-purple-300"
                          >
                            <Download className="h-3 w-3" />
                          </Button>
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleDeleteDocument(doc.id);
                            }}
                            className="h-6 w-6 p-0 text-red-400 hover:text-red-300"
                          >
                            <Trash2 className="h-3 w-3" />
                          </Button>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Document Viewer/Editor */}
          <div className="lg:col-span-2">
            {selectedDoc ? (
              <Card className="bg-slate-800/50 border-purple-500/20">
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <CardTitle className="text-white flex items-center">
                      <Eye className="h-5 w-5 mr-2" />
                      {selectedDoc.name}
                    </CardTitle>
                    <div className="flex gap-2">
                      {isEditing ? (
                        <>
                          <Button
                            size="sm"
                            onClick={handleSaveEdit}
                            className="bg-green-600 hover:bg-green-700"
                          >
                            <Save className="h-4 w-4 mr-1" />
                            Save
                          </Button>
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => setIsEditing(false)}
                          >
                            <X className="h-4 w-4 mr-1" />
                            Cancel
                          </Button>
                        </>
                      ) : (
                        <Button
                          size="sm"
                          onClick={() => setIsEditing(true)}
                          className="bg-purple-600 hover:bg-purple-700"
                        >
                          <Edit className="h-4 w-4 mr-1" />
                          Edit
                        </Button>
                      )}
                    </div>
                  </div>
                </CardHeader>
                <CardContent>
                  {isEditing ? (
                    <Textarea
                      value={editContent}
                      onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => setEditContent(e.target.value)}
                      className="min-h-96 bg-slate-900/50 border-purple-500/20 text-white font-mono text-sm"
                      placeholder="Edit markdown content..."
                    />
                  ) : (
                    <div className="bg-slate-900/50 border border-purple-500/20 rounded-lg p-4">
                      <pre className="text-purple-100 text-sm whitespace-pre-wrap font-mono overflow-auto max-h-96">
                        {selectedDoc.content}
                      </pre>
                    </div>
                  )}
                </CardContent>
              </Card>
            ) : (
              <Card className="bg-slate-800/50 border-purple-500/20">
                <CardContent className="p-12 text-center">
                  <FileText className="h-16 w-16 text-purple-400 mx-auto mb-4" />
                  <h3 className="text-xl font-semibold text-white mb-2">
                    Select a Document
                  </h3>
                  <p className="text-purple-200">
                    Choose a documentation file from the list to view or edit its content.
                  </p>
                </CardContent>
              </Card>
            )}
          </div>
        </div>

        {/* Add Document Modal */}
        {showAddForm && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
            <Card className="bg-slate-800 border-purple-500/20 w-full max-w-2xl">
              <CardHeader>
                <CardTitle className="text-white flex items-center justify-between">
                  <span>Add New Document</span>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => setShowAddForm(false)}
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div>
                  <label className="text-purple-200 text-sm mb-2 block">
                    Document Name
                  </label>
                  <Input
                    value={newDocName}
                    onChange={(e) => setNewDocName(e.target.value)}
                    placeholder="e.g., 11-NEW_DOCUMENT.md"
                    className="bg-slate-900/50 border-purple-500/20 text-white"
                  />
                </div>
                <div>
                  <label className="text-purple-200 text-sm mb-2 block">
                    Description
                  </label>
                  <Input
                    value={newDocDescription}
                    onChange={(e) => setNewDocDescription(e.target.value)}
                    placeholder="Brief description of the document"
                    className="bg-slate-900/50 border-purple-500/20 text-white"
                  />
                </div>
                <div>
                  <label className="text-purple-200 text-sm mb-2 block">
                    Content
                  </label>
                  <Textarea
                    value={newDocContent}
                    onChange={(e) => setNewDocContent(e.target.value)}
                    placeholder="# Document Title

## Section 1

Your markdown content here..."
                    className="min-h-48 bg-slate-900/50 border-purple-500/20 text-white font-mono"
                  />
                </div>
                <div className="flex gap-3 pt-4">
                  <Button
                    onClick={handleAddDocument}
                    className="bg-purple-600 hover:bg-purple-700 flex-1"
                  >
                    <Plus className="h-4 w-4 mr-2" />
                    Add Document
                  </Button>
                  <Button
                    variant="outline"
                    onClick={() => setShowAddForm(false)}
                    className="flex-1"
                  >
                    Cancel
                  </Button>
                </div>
              </CardContent>
            </Card>
          </div>
        )}
      </div>
    </div>
  );
};

export default DocumentationManager;
