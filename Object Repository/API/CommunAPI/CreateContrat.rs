<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateContrat</name>
   <tag></tag>
   <elementGuidId>8034a11c-0ee0-4c78-9954-cf6aa3dd1b61</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;userAccount\&quot;: \&quot;${codeCopropriete}\&quot;,\n    \&quot;offerTemplate\&quot;: \&quot;OFFER_EAU_LER\&quot;,\n    \&quot;renewalRule\&quot;: {\n        \&quot;calendarInitialyActiveFor\&quot;: {}\n    },\n    \&quot;subscriptionDate\&quot;:\&quot;{dateSouscription}\&quot;,\n    \&quot;seller\&quot;: \&quot;AGENCE_Toulouse\&quot;,\n    \&quot;customFields\&quot;: {\n        \&quot;customField\&quot;: [\n            {\n                \&quot;code\&quot;: \&quot;CF_ARRONDI_FACTURE\&quot;,\n                \&quot;longValue\&quot;: 4\n            },\n                      {\n                \&quot;code\&quot;: \&quot;CF_MODE_ARRONDI\&quot;,\n                \&quot;stringValue\&quot;: \&quot;Au supérieur\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_CODE_ANALYTIQUE_AFFAIRE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;999997\&quot;\n            }\n        ]\n    },\n    \&quot;salesPersonName\&quot;: \&quot;\&quot;,\n    \&quot;discountPlanForInstantiation\&quot;: [],\n    \&quot;discountPlanForTermination\&quot;: [],\n    \&quot;priceListCode\&quot;: \&quot;\&quot;,\n    \&quot;paymentMethod\&quot;: null,\n    \&quot;productToInstantiateDto\&quot;: [\n      \t      \t{\n            \&quot;productCode\&quot;: \&quot;PROD_FLCMP\&quot;,\n            \&quot;quantity\&quot;: 5,\n            \&quot;deliveryDate\&quot;: \&quot;${dateDeLivraison}\&quot;,\n            \&quot;attributeInstances\&quot;: [\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Code_Revision\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Nbre_Compteurs\&quot;,\n                    \&quot;doubleValue\&quot;: 10,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n              \t                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Nbre_APP\&quot;,\n                    \&quot;doubleValue\&quot;: 15,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Technologie\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;RADIO\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n              \t{\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Forfaitaire\&quot;,\n                    \&quot;stringValue\&quot;: true,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Revise\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Fluide\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;EC\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Libelle\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;,\n                  \t\&quot;stringValue\&quot;: \&quot;Juste Pour faire le Test\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Initial\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;doubleValue\&quot;: 4.999,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                }\n            ]\n        }\n    ],\n    \&quot;validityDate\&quot;: 1741171420908,\n    \&quot;code\&quot;: \&quot;${codeContrat}\&quot;\n}\n&quot;,
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
      <webElementGuid>385c3c85-d4f9-4e04-b174-e3bdaf2029c9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic b3BlbmNlbGwuYWRtaW46b3BlbmNlbGwuYWRtaW4=</value>
      <webElementGuid>f41a2287-b19a-467e-be2d-2b9dbb769637</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL_API}/billing/subscription/subscribeAndInstantiateProducts</restUrl>
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
      <id>9c89bf6c-b123-42f8-a75d-86b10625f1df</id>
      <masked>false</masked>
      <name>dateDeLivraison</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e002b4dd-e630-4e43-b33c-35bb7688aa3a</id>
      <masked>false</masked>
      <name>codeCopropriete</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>36f96eaf-8a05-4719-a31c-135dd3b0bba3</id>
      <masked>false</masked>
      <name>dateSouscription</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>06e27dcf-d312-494a-bc87-1c6d402566b3</id>
      <masked>false</masked>
      <name>codeContrat</name>
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
