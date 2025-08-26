<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Gestionnaire</name>
   <tag></tag>
   <elementGuidId>93d4bf73-e77b-49bc-a484-d9a11f311250</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;crmAccountType\&quot;: \&quot;C\&quot;,\n    \&quot;code\&quot;: \&quot;${GEST_codeGestionnaire}\&quot;,\n    \&quot;description\&quot;: \&quot;${descriptionName}\&quot;,\n    \&quot;externalRef1\&quot;: \&quot;EXT001\&quot;,\n    \&quot;externalRef2\&quot;: \&quot;EXT002\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address\&quot;: {\n        \&quot;address1\&quot;: \&quot;${address1}\&quot;,\n        \&quot;address2\&quot;: \&quot;${address2}\&quot;,\n        \&quot;address3\&quot;: \&quot;${address3}\&quot;,\n        \&quot;zipCode\&quot;: \&quot;${zipCode}\&quot;,\n        \&quot;city\&quot;: \&quot;NARBONNE CEDEX\&quot;,\n        \&quot;country\&quot;: \&quot;France\&quot;,\n        \&quot;state\&quot;: \&quot;Ile-de-France\&quot;\n    },\n    \&quot;contactInformation\&quot;: {\n        \&quot;email\&quot;: \&quot;${infoEmail}\&quot;,\n        \&quot;phone\&quot;: \&quot;+33123450009\&quot;,\n        \&quot;mobile\&quot;: \&quot;+33612340008\&quot;,\n        \&quot;fax\&quot;: \&quot;711100005\&quot;\n    },\n    \&quot;jobTitle\&quot;: \&quot;Manager\&quot;,\n    \&quot;language\&quot;: \&quot;FRA\&quot;,\n    \&quot;customerCategory\&quot;: \&quot;Syndic\&quot;,\n    \&quot;seller\&quot;: \&quot;AGENCE_Toulouse\&quot;,\n    \n    \&quot;registrationNumbers\&quot;: [\n        {\n            \&quot;registrationNo\&quot;: \&quot;12342222900001\&quot;,\n            \&quot;isoIcdCode\&quot;: \&quot;0009\&quot;\n        }\n    ],\n    \&quot;vatNo\&quot;: \&quot;FR12345678111\&quot;,\n    \&quot;mandateIdentification\&quot;: \&quot;MANDAT001\&quot;,\n    \&quot;mandateDate\&quot;: \&quot;2025-02-03\&quot;,\n    \&quot;currency\&quot;: \&quot;EUR\&quot;,\n    \&quot;creditCategory\&quot;: \&quot;PRO\&quot;,\n    \&quot;isCompany\&quot;: true,\n    \&quot;country\&quot;: \&quot;FR\&quot;,\n       \&quot;customFields\&quot;: {\n        \&quot;customField\&quot;: [\n            {\n                \&quot;code\&quot;: \&quot;CF_CODE_COMMERCIAL\&quot;,\n                \&quot;stringValue\&quot;: \&quot;COM2025-123\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_FOURNISSEURS\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Freedz\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_NOM_INTERLOCUTEUR\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Marie Dubois\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_ADRESSE_EMAIL_DESTINATION\&quot;,\n                \&quot;stringValue\&quot;: \&quot;facturation@entreprise.fr\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_EMAIL_INTERLOCUTEUR\&quot;,\n                \&quot;stringValue\&quot;: \&quot;m.dubois@entreprise.fr\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_MODE_ENVOI_FACTURE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;EDI\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_STATUT_VIP\&quot;,\n                \&quot;booleanValue\&quot;: true\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_OPTION_DE_REGROUPEMENT\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Grouper la facture et ses pièces jointes\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_NOM_COMMERCIAL\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Pierre Martin\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_PARAMETRE_EDI_SPECIFIQUE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;EDI_PARAM_456\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_DELAI_REGLEMENT\&quot;,\n                \&quot;stringValue\&quot;: \&quot;${delaisReglement}\&quot;\n            }\n        ]\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9012e2aa-7aa5-4b51-b09c-9b4b747b54d1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic b3BlbmNlbGwuYWRtaW46b3BlbmNlbGwuYWRtaW4=</value>
      <webElementGuid>030d8082-5c53-4e35-bcd8-0c8e2e41677d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL_API}/account/accountHierarchy/createOrUpdateCRMAccountHierarchy</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d566349b-0e44-4666-a6e7-039b103e12bf</id>
      <masked>false</masked>
      <name>descriptionName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>eb21e324-5bae-4786-8c24-d97a1bc8b7a0</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>44cad52e-377e-46c7-81f3-1b4f5c2b5219</id>
      <masked>false</masked>
      <name>infoEmail</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a96bc39c-1c6f-4861-a792-2072dcf2e63f</id>
      <masked>false</masked>
      <name>zipCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b47b34e9-1015-43e2-8add-b1a632b6cf0b</id>
      <masked>false</masked>
      <name>address1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f8747a41-b5e8-4b59-993b-9b2ed8e41644</id>
      <masked>false</masked>
      <name>address2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ef21750c-538e-4324-85ce-211ed67d2998</id>
      <masked>false</masked>
      <name>address3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5b930a47-9d90-4b46-a727-80b48eceabc5</id>
      <masked>false</masked>
      <name>GEST_codeGestionnaire</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a273ff4e-7568-4d99-bc20-e706c8bac06e</id>
      <masked>false</masked>
      <name>delaisReglement</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
